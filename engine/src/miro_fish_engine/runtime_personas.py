from __future__ import annotations

import re
from collections import Counter
from dataclasses import dataclass
from pathlib import Path

from pydantic import BaseModel, Field

from .moderator_basis import ModeratorBasis, build_moderator_basis
from .openai_client import MiroFishOpenAIClient
from .prompts import PERSONA_EXTRACTION_SYSTEM_PROMPT, PERSONA_EXTRACTION_USER_PROMPT

STOP_WORDS = {
    "about",
    "after",
    "also",
    "been",
    "between",
    "could",
    "from",
    "have",
    "into",
    "more",
    "should",
    "than",
    "that",
    "their",
    "there",
    "these",
    "this",
    "through",
    "what",
    "when",
    "where",
    "which",
    "while",
    "with",
    "would",
}


@dataclass(frozen=True)
class SeedContext:
    source_name: str
    source_kind: str
    fingerprint_sha256: str
    text_excerpt: str
    topic_keywords: tuple[str, ...]
    participant_count: int


@dataclass(frozen=True)
class RuntimePersona:
    stable_label: str
    role: str
    perspective: str
    focus_note: str
    seed_grounding_note: str


class LLMExtractedPersona(BaseModel):
    """Pydantic model for structured persona extraction from OpenAI."""
    label: str = Field(description="Alphanumeric persona title, e.g. 'Security Auditor'")
    perspective: str = Field(
        description="Classification: 'favorable', 'skeptical', 'analytical', or 'neutral'"
    )
    focus_note: str = Field(description="Core question or strategic angle pursued")
    critique_vector: str = Field(description="Key risk or blindspot this persona evaluates")


class LLMPersonaRoster(BaseModel):
    """Structured response format for OpenAI chat.completions.parse."""
    personas: list[LLMExtractedPersona] = Field(
        description="List of 4 to 6 diverse, domain-relevant personas"
    )


@dataclass(frozen=True)
class RuntimePersonaFoundation:
    moderator_basis: ModeratorBasis
    moderator: RuntimePersona
    participants: tuple[RuntimePersona, ...]

    @property
    def all_personas(self) -> tuple[RuntimePersona, ...]:
        return (self.moderator, *self.participants)

    def stable_labels(self) -> tuple[str, ...]:
        return tuple(persona.stable_label for persona in self.all_personas)

    def perspective_coverage(self) -> tuple[str, ...]:
        return tuple(dict.fromkeys(persona.perspective for persona in self.participants))


def derive_seed_context(
    source_name: str,
    source_kind: str,
    fingerprint_sha256: str,
    raw_text: str,
    participant_count: int,
) -> SeedContext:
    normalized_text = normalize_whitespace(raw_text)
    excerpt = normalized_text[:1200]
    keywords = extract_topic_keywords(normalized_text, source_name)
    return SeedContext(
        source_name=source_name,
        source_kind=source_kind,
        fingerprint_sha256=fingerprint_sha256,
        text_excerpt=excerpt,
        topic_keywords=keywords,
        participant_count=max(participant_count, 3),
    )


def build_runtime_persona_foundation(seed_context: SeedContext) -> RuntimePersonaFoundation:
    moderator_basis = build_moderator_basis(seed_context)
    topic_phrase = ", ".join(seed_context.topic_keywords[:3]) or stemmed_name(
        seed_context.source_name
    )
    grounding_note = (
        f"Seed grounding uses {seed_context.source_kind} context '{seed_context.source_name}' "
        f"with fingerprint {seed_context.fingerprint_sha256[:12]}."
    )
    moderator = RuntimePersona(
        stable_label=moderator_basis.stable_label,
        role=moderator_basis.role,
        perspective=moderator_basis.perspective,
        focus_note=(
            f"Frame the round around {topic_phrase} and enforce the moderated 3-to-5 speaker rule."
        ),
        seed_grounding_note=grounding_note,
    )

    participants = tuple(
        _build_participant_persona(seed_context, topic_phrase, grounding_note, index)
        for index in range(seed_context.participant_count)
    )

    foundation = RuntimePersonaFoundation(
        moderator_basis=moderator_basis,
        moderator=moderator,
        participants=participants,
    )
    validate_runtime_persona_foundation(foundation)
    return foundation


async def build_runtime_persona_foundation_with_openai(
    seed_context: SeedContext,
    client: MiroFishOpenAIClient,
) -> RuntimePersonaFoundation:
    """Asynchronously extracts structured stakeholder personas from seed document
    using OpenAI Structured Outputs.
    """
    if not client.is_configured:
        return build_runtime_persona_foundation(seed_context)

    moderator_basis = build_moderator_basis(seed_context)
    topic_phrase = ", ".join(seed_context.topic_keywords[:3]) or stemmed_name(
        seed_context.source_name
    )
    grounding_note = (
        f"Seed grounding uses {seed_context.source_kind} context '{seed_context.source_name}' "
        f"with fingerprint {seed_context.fingerprint_sha256[:12]}."
    )
    moderator = RuntimePersona(
        stable_label=moderator_basis.stable_label,
        role=moderator_basis.role,
        perspective=moderator_basis.perspective,
        focus_note=(
            f"Frame the round around {topic_phrase} and enforce the moderated 3-to-5 speaker rule."
        ),
        seed_grounding_note=grounding_note,
    )

    user_prompt = PERSONA_EXTRACTION_USER_PROMPT.format(seed_excerpt=seed_context.text_excerpt)
    roster, _ = await client.parse_structured_output(
        response_format=LLMPersonaRoster,
        system_prompt=PERSONA_EXTRACTION_SYSTEM_PROMPT,
        user_prompt=user_prompt,
    )

    participants_list: list[RuntimePersona] = []
    seen_labels: set[str] = {moderator.stable_label}
    for item in roster.personas[: seed_context.participant_count]:
        clean_label = item.label.strip()
        if clean_label in seen_labels:
            clean_label = f"{clean_label} (Alt)"
        seen_labels.add(clean_label)

        perspective = item.perspective.lower().strip()
        if perspective not in {"favorable", "skeptical", "analytical"}:
            perspective = "analytical"

        participants_list.append(
            RuntimePersona(
                stable_label=clean_label,
                role="participant",
                perspective=perspective,
                focus_note=f"{item.focus_note}. Critique vector: {item.critique_vector}",
                seed_grounding_note=grounding_note,
            )
        )

    # Ensure required perspective diversity is satisfied
    perspectives = {p.perspective for p in participants_list}
    for req in ["favorable", "skeptical", "analytical"]:
        if req not in perspectives:
            blueprints = _participant_blueprints(topic_phrase)
            for b_label, b_persp, b_focus in blueprints:
                if b_persp == req:
                    label = f"{b_label}_{req}"
                    if label in seen_labels:
                        label = f"{label}_auto"
                    seen_labels.add(label)
                    participants_list.append(
                        RuntimePersona(
                            stable_label=label,
                            role="participant",
                            perspective=req,
                            focus_note=b_focus.format(
                                topic_phrase=topic_phrase, source_name=seed_context.source_name
                            ),
                            seed_grounding_note=grounding_note,
                        )
                    )
                    break
            perspectives.add(req)

    foundation = RuntimePersonaFoundation(
        moderator_basis=moderator_basis,
        moderator=moderator,
        participants=tuple(participants_list),
    )
    validate_runtime_persona_foundation(foundation)
    return foundation


def validate_runtime_persona_foundation(foundation: RuntimePersonaFoundation) -> None:
    stable_labels = foundation.stable_labels()
    if len(stable_labels) != len(set(stable_labels)):
        raise ValueError("runtime persona labels must remain stable and unique within the run")

    if foundation.moderator.role != "moderator":
        raise ValueError("exactly one Moderator role is required")

    required_perspectives = {"favorable", "skeptical", "analytical"}
    if not required_perspectives.issubset(set(foundation.perspective_coverage())):
        raise ValueError("runtime persona set is missing required perspective diversity")


def normalize_whitespace(value: str) -> str:
    return re.sub(r"\s+", " ", value).strip()


def extract_topic_keywords(raw_text: str, source_name: str, limit: int = 4) -> tuple[str, ...]:
    normalized = normalize_whitespace(raw_text)
    tokens = [
        match.group(0).lower()
        for match in re.finditer(r"[A-Za-z][A-Za-z\-']{3,}", normalized)
        if match.group(0).lower() not in STOP_WORDS
    ]
    counts = Counter(tokens)
    ordered_tokens = sorted(counts.items(), key=lambda item: (-item[1], tokens.index(item[0])))
    keywords = tuple(token for token, _count in ordered_tokens[:limit])
    if keywords:
        return keywords

    fallback = stemmed_name(source_name).lower()
    return (fallback,) if fallback else ("seed-context",)


def stemmed_name(source_name: str) -> str:
    return Path(source_name).stem.replace("_", " ").replace("-", " ").strip() or "seed context"


def _build_participant_persona(
    seed_context: SeedContext,
    topic_phrase: str,
    grounding_note: str,
    index: int,
) -> RuntimePersona:
    blueprints = _participant_blueprints(topic_phrase)
    label, perspective, focus_template = blueprints[index % len(blueprints)]
    label_suffix = "" if index < len(blueprints) else f" {index + 1}"
    stable_label = f"{label}{label_suffix}"
    focus_note = focus_template.format(
        topic_phrase=topic_phrase, source_name=seed_context.source_name
    )
    return RuntimePersona(
        stable_label=stable_label,
        role="participant",
        perspective=perspective,
        focus_note=focus_note,
        seed_grounding_note=grounding_note,
    )


def _participant_blueprints(topic_phrase: str) -> tuple[tuple[str, str, str], ...]:
    return (
        (
            "Advocate",
            "favorable",
            "Surface the upside, strategic leverage, and high-conviction opportunity in "
            "{topic_phrase}.",
        ),
        (
            "Skeptic",
            "skeptical",
            "Stress-test hidden assumptions, failure modes, and downside risk in {topic_phrase}.",
        ),
        (
            "Analyst",
            "analytical",
            "Separate evidence from conjecture and identify what must be proved about "
            "{topic_phrase}.",
        ),
        (
            "Builder",
            "neutral",
            "Translate the seed context into actionable next moves and implementation posture "
            "for {topic_phrase}.",
        ),
        (
            "Auditor",
            "skeptical",
            "Audit governance, compliance, and boundary risks implied by {topic_phrase}.",
        ),
        (
            "Synthesizer",
            "analytical",
            "Reconcile disagreement into crisp tradeoffs while staying grounded in {topic_phrase}.",
        ),
    )

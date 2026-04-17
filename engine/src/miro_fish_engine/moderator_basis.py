from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .runtime_personas import SeedContext


@dataclass(frozen=True)
class ModeratorBasis:
    stable_label: str
    role: str
    perspective: str
    selection_policy_name: str
    normal_round_min_speakers: int
    normal_round_max_speakers: int
    seed_focus: str
    guidance_note: str


def build_moderator_basis(seed_context: "SeedContext") -> ModeratorBasis:
    topic_phrase = ", ".join(seed_context.topic_keywords[:3]) or seed_context.source_name
    return ModeratorBasis(
        stable_label="Moderator",
        role="moderator",
        perspective="moderator",
        selection_policy_name="moderated_3_to_5_speakers_per_round",
        normal_round_min_speakers=3,
        normal_round_max_speakers=5,
        seed_focus=topic_phrase,
        guidance_note=(
            "Keep the run grounded in the approved seed context, preserve perspective diversity, "
            "and prepare bounded round continuity without using transcript replay as "
            "unlimited prompt state."
        ),
    )

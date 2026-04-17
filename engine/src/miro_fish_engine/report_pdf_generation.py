from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

from reportlab.lib.pagesizes import LETTER
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import PageBreak, Paragraph, SimpleDocTemplate, Spacer

GENERATED_REPORT_PDF_FILE_NAME = "final_report.pdf"

FAILURE_REASON_MISSING_REPORT_DATA = "missing_report_data"
FAILURE_REASON_MALFORMED_REPORT_DATA = "malformed_report_data"
FAILURE_REASON_PDF_GENERATION_FAILED = "pdf_generation_failed"


class ReportPdfGenerationError(RuntimeError):
    def __init__(self, failure_reason: str, detail_message: str) -> None:
        super().__init__(detail_message)
        self.failure_reason = failure_reason
        self.detail_message = detail_message


@dataclass(frozen=True)
class ReportPdfArtifact:
    report_identifier: str
    report_pdf_path: str
    generated_at: int
    detail_message: str


def generate_report_pdf_artifact(
    *,
    report_data_path: Path,
    output_pdf_path: Path,
) -> ReportPdfArtifact:
    try:
        report_payload = json.loads(report_data_path.read_text(encoding="utf-8"))
    except FileNotFoundError as error:
        raise ReportPdfGenerationError(
            FAILURE_REASON_MISSING_REPORT_DATA,
            f"Report PDF generation could not find assembled report data: {error.filename}",
        ) from error
    except json.JSONDecodeError as error:
        raise ReportPdfGenerationError(
            FAILURE_REASON_MALFORMED_REPORT_DATA,
            f"Report PDF generation could not decode assembled report data: {error}",
        ) from error

    metadata = report_payload.get("report_metadata")
    sections = report_payload.get("report_sections")
    if not isinstance(metadata, dict) or not isinstance(sections, dict):
        raise ReportPdfGenerationError(
            FAILURE_REASON_MALFORMED_REPORT_DATA,
            "Report PDF generation requires report metadata and section payloads.",
        )

    report_identifier = string_field(metadata, "report_identifier")
    generated_at = int_field(metadata, "generated_at")

    styles = build_styles()
    story = build_story(report_identifier, metadata, sections, styles)

    try:
        output_pdf_path.parent.mkdir(parents=True, exist_ok=True)
        document = SimpleDocTemplate(
            str(output_pdf_path),
            pagesize=LETTER,
            leftMargin=0.75 * inch,
            rightMargin=0.75 * inch,
            topMargin=0.75 * inch,
            bottomMargin=0.75 * inch,
            title=report_identifier,
        )
        document.build(story)
    except (OSError, TypeError, ValueError) as error:
        raise ReportPdfGenerationError(
            FAILURE_REASON_PDF_GENERATION_FAILED,
            f"Report PDF generation could not write the PDF artifact: {error}",
        ) from error

    return ReportPdfArtifact(
        report_identifier=report_identifier,
        report_pdf_path=str(output_pdf_path),
        generated_at=generated_at,
        detail_message=(
            "Required report sections were rendered into a governed PDF artifact at "
            f"{output_pdf_path.name}."
        ),
    )


def build_styles() -> dict[str, ParagraphStyle]:
    stylesheet = getSampleStyleSheet()
    return {
        "title": ParagraphStyle(
            "ReportTitle",
            parent=stylesheet["Title"],
            fontSize=20,
            leading=24,
            spaceAfter=12,
        ),
        "heading": ParagraphStyle(
            "ReportHeading",
            parent=stylesheet["Heading2"],
            fontSize=14,
            leading=18,
            spaceBefore=10,
            spaceAfter=6,
        ),
        "body": ParagraphStyle(
            "ReportBody",
            parent=stylesheet["BodyText"],
            fontSize=10,
            leading=14,
            spaceAfter=6,
        ),
        "meta": ParagraphStyle(
            "ReportMeta",
            parent=stylesheet["BodyText"],
            fontSize=9,
            leading=12,
            textColor="#444444",
            spaceAfter=4,
        ),
    }


def build_story(
    report_identifier: str,
    metadata: dict[str, object],
    sections: dict[str, object],
    styles: dict[str, ParagraphStyle],
) -> list[object]:
    story: list[object] = [
        Paragraph("Miro Fish Report", styles["title"]),
        Paragraph(f"Report identifier: {escape_text(report_identifier)}", styles["meta"]),
        Paragraph(
            (
                "Completion classification: "
                f"{escape_text(string_field(metadata, 'run_outcome_classification'))}"
            ),
            styles["meta"],
        ),
        Paragraph(
            (
                "Generated at epoch ms: "
                f"{int_field(metadata, 'generated_at')}"
            ),
            styles["meta"],
        ),
        Spacer(1, 0.12 * inch),
    ]

    executive_summary = dict_field(sections, "executive_summary")
    story.extend(
        [
            Paragraph("Executive Summary", styles["heading"]),
            Paragraph(
                escape_text(string_field(executive_summary, "high_level_synthesis_body")),
                styles["body"],
            ),
        ]
    )
    append_optional_paragraph(
        story,
        executive_summary.get("summary_scope_note"),
        styles["body"],
    )
    append_optional_paragraph(
        story,
        executive_summary.get("completed_material_note"),
        styles["body"],
    )

    viability_score = dict_field(sections, "market_viability_score")
    story.extend(
        [
            Paragraph("Market Viability Score", styles["heading"]),
            Paragraph(
                escape_text(
                    " ".join(
                        [
                            f"Label: {string_field(viability_score, 'score_label')}",
                            (
                                "Score: "
                                f"{int_field(viability_score, 'score_value')} "
                                f"({int_field(viability_score, 'score_range_min')}"
                                f"-{int_field(viability_score, 'score_range_max')})"
                            ),
                            (
                                "Scoring actor: "
                                f"{string_field(viability_score, 'scoring_actor_reference')}"
                            ),
                        ]
                    )
                ),
                styles["body"],
            ),
        ]
    )
    append_optional_paragraph(
        story,
        viability_score.get("constrained_context_note"),
        styles["body"],
    )

    ranked_risks = dict_field(sections, "ranked_risks")
    story.append(Paragraph("Ranked Risks", styles["heading"]))
    for risk in list_field(ranked_risks, "ranked_risks"):
        risk_entry = dict_value(risk, "ranked risk entry")
        story.append(
            Paragraph(
                escape_text(
                    (
                        f"{int_field(risk_entry, 'rank')}. "
                        f"{string_field(risk_entry, 'risk_title')} "
                        f"[{string_field(risk_entry, 'risk_type')}]. "
                        f"{string_field(risk_entry, 'risk_description')} "
                        f"{string_field(risk_entry, 'emphasis_note')}"
                    )
                ),
                styles["body"],
            )
        )
    append_optional_paragraph(story, ranked_risks.get("completeness_note"), styles["body"])

    story.extend(
        build_argument_section(
            "Strongest Pro Arguments",
            sections,
            "strongest_pro_arguments",
            styles,
        )
    )
    story.extend(
        build_argument_section(
            "Strongest Anti Arguments",
            sections,
            "strongest_anti_arguments",
            styles,
        )
    )

    cost_summary = dict_field(sections, "cost_and_token_summary")
    requested_token_budget = optional_int_field(cost_summary, "token_budget_requested")
    story.extend(
        [
            Paragraph("Cost And Token Summary", styles["heading"]),
            Paragraph(
                escape_text(
                    " ".join(
                        [
                            (
                                "Requested token budget: "
                                f"{requested_token_budget or 'Not reported'}"
                            ),
                            (
                                "Tracked token usage: "
                                f"{int_field(cost_summary, 'token_usage_total')}"
                            ),
                            (
                                "Duration elapsed ms: "
                                f"{int_field(cost_summary, 'duration_elapsed_reference_ms')}"
                            ),
                            (
                                "Cost note: "
                                f"{string_field(cost_summary, 'cost_currency_or_cost_unit_note')}"
                            ),
                        ]
                    )
                ),
                styles["body"],
            ),
        ]
    )
    append_optional_paragraph(
        story,
        cost_summary.get("constrained_stop_reason_reference"),
        styles["body"],
    )

    story.extend([PageBreak(), Paragraph("Transcript Appendix", styles["heading"])])
    transcript_appendix = dict_field(sections, "transcript_appendix")
    for entry in list_field(transcript_appendix, "ordered_transcript_entries"):
        appendix_entry = dict_value(entry, "transcript appendix entry")
        story.append(
            Paragraph(
                escape_text(
                    (
                        f"Seq {int_field(appendix_entry, 'sequence_reference')} | "
                        f"Round {int_field(appendix_entry, 'round_number')} | "
                        f"Turn {int_field(appendix_entry, 'turn_index')} | "
                        f"{string_field(appendix_entry, 'speaker_reference')}: "
                        f"{string_field(appendix_entry, 'event_body_content')}"
                    )
                ),
                styles["body"],
            )
        )
    append_optional_paragraph(
        story,
        transcript_appendix.get("appendix_completeness_note"),
        styles["body"],
    )

    return story


def build_argument_section(
    title: str,
    sections: dict[str, object],
    section_name: str,
    styles: dict[str, ParagraphStyle],
) -> list[object]:
    argument_section = dict_field(sections, section_name)
    story: list[object] = [Paragraph(title, styles["heading"])]
    for entry in list_field(argument_section, "arguments"):
        argument_entry = dict_value(entry, "argument entry")
        story.append(
            Paragraph(
                escape_text(
                    (
                        f"{string_field(argument_entry, 'argument_title')}: "
                        f"{string_field(argument_entry, 'argument_body')} "
                        f"{string_field(argument_entry, 'supporting_debate_linkage_note')}"
                    )
                ),
                styles["body"],
            )
        )
    append_optional_paragraph(story, argument_section.get("completeness_note"), styles["body"])
    return story


def append_optional_paragraph(
    story: list[object],
    value: object,
    style: ParagraphStyle,
) -> None:
    if value is None:
        return
    text = str(value).strip()
    if text:
        story.append(Paragraph(escape_text(text), style))


def dict_field(payload: dict[str, object], field_name: str) -> dict[str, object]:
    value = payload.get(field_name)
    return dict_value(value, field_name)


def dict_value(value: object, label: str) -> dict[str, object]:
    if not isinstance(value, dict):
        raise ReportPdfGenerationError(
            FAILURE_REASON_MALFORMED_REPORT_DATA,
            f"Report PDF generation expected {label} to be an object.",
        )
    return value


def list_field(payload: dict[str, object], field_name: str) -> list[object]:
    value = payload.get(field_name)
    if not isinstance(value, list):
        raise ReportPdfGenerationError(
            FAILURE_REASON_MALFORMED_REPORT_DATA,
            f"Report PDF generation expected {field_name} to be a list.",
        )
    return value


def string_field(payload: dict[str, object], field_name: str) -> str:
    value = payload.get(field_name)
    if not isinstance(value, str) or not value.strip():
        raise ReportPdfGenerationError(
            FAILURE_REASON_MALFORMED_REPORT_DATA,
            f"Report PDF generation expected {field_name} to be a non-empty string.",
        )
    return value.strip()


def int_field(payload: dict[str, object], field_name: str) -> int:
    value = payload.get(field_name)
    if not isinstance(value, int):
        raise ReportPdfGenerationError(
            FAILURE_REASON_MALFORMED_REPORT_DATA,
            f"Report PDF generation expected {field_name} to be an integer.",
        )
    return value


def optional_int_field(payload: dict[str, object], field_name: str) -> int | None:
    value = payload.get(field_name)
    if value is None:
        return None
    if not isinstance(value, int):
        raise ReportPdfGenerationError(
            FAILURE_REASON_MALFORMED_REPORT_DATA,
            f"Report PDF generation expected {field_name} to be an integer or null.",
        )
    return value


def escape_text(text: str) -> str:
    return (
        text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\n", "<br/>")
    )
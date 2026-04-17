from __future__ import annotations

import json
from pathlib import Path

from pypdf import PdfReader

from miro_fish_engine.report_pdf_generation import generate_report_pdf_artifact


def test_generate_report_pdf_artifact_renders_governed_sections(tmp_path: Path) -> None:
    report_data_path = tmp_path / "assembled_report_data.json"
    pdf_output_path = tmp_path / "final_report.pdf"
    report_data_path.write_text(
        json.dumps(
            {
                "report_data_version": "phase8_report_data_v1",
                "report_metadata": {
                    "report_identifier": "report-data-run-1-1000",
                    "run_outcome_classification": "completed",
                    "report_generation_status": "succeeded",
                    "report_availability_status": "report_unavailable",
                    "generated_at": 1000,
                    "constrained_completion_flag": False,
                    "token_cost_summary_reference": "#/report_sections/cost_and_token_summary",
                    "artifact_location_reference": str(report_data_path),
                },
                "report_sections": {
                    "executive_summary": {
                        "summary_heading": "Executive Summary",
                        "high_level_synthesis_body": "A bounded run produced a grounded synthesis.",
                        "summary_scope_note": None,
                        "completed_material_note": "Only actual run material is included.",
                    },
                    "market_viability_score": {
                        "score_label": "Promising",
                        "score_value": 68,
                        "score_range_min": 63,
                        "score_range_max": 73,
                        "scoring_actor_reference": "Moderator",
                        "constrained_context_note": None,
                    },
                    "ranked_risks": {
                        "ranked_risks": [
                            {
                                "rank": 1,
                                "risk_title": "Demand proof remains thin",
                                "risk_type": "market",
                                "risk_description": (
                                    "Demand proof remains thin without wider trials."
                                ),
                                "originating_actor_class_reference": "Skeptic",
                                "emphasis_note": "Actual continuity material only.",
                            }
                        ],
                        "completeness_note": None,
                    },
                    "strongest_pro_arguments": {
                        "arguments": [
                            {
                                "argument_title": "Clear wedge",
                                "argument_body": (
                                    "The product has a narrow but defensible wedge."
                                ),
                                "argument_side": "pro",
                                "source_actor_class_reference": "Advocate",
                                "supporting_debate_linkage_note": (
                                    "Derived from actual bounded continuity."
                                ),
                            }
                        ],
                        "completeness_note": None,
                    },
                    "strongest_anti_arguments": {
                        "arguments": [
                            {
                                "argument_title": "Go-to-market risk",
                                "argument_body": (
                                    "Customer acquisition assumptions remain unproven."
                                ),
                                "argument_side": "anti",
                                "source_actor_class_reference": "Skeptic",
                                "supporting_debate_linkage_note": (
                                    "Derived from actual bounded continuity."
                                ),
                            }
                        ],
                        "completeness_note": None,
                    },
                    "cost_and_token_summary": {
                        "token_budget_requested": 12000,
                        "token_usage_total": 610,
                        "cost_estimate_or_actual_cost_value": None,
                        "cost_currency_or_cost_unit_note": "Tracked local token units only.",
                        "duration_elapsed_reference_ms": 48000,
                        "constrained_stop_reason_reference": None,
                    },
                    "transcript_appendix": {
                        "ordered_transcript_entries": [
                            {
                                "sequence_reference": 1,
                                "ordered_occurrence_reference": 1000,
                                "round_number": 1,
                                "turn_index": 1,
                                "speaker_reference": "Moderator",
                                "speaker_perspective": "neutral",
                                "event_body_content": (
                                    "Frame the key market and execution tradeoffs."
                                ),
                            }
                        ],
                        "appendix_completeness_note": None,
                    },
                },
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    artifact = generate_report_pdf_artifact(
        report_data_path=report_data_path,
        output_pdf_path=pdf_output_path,
    )

    assert artifact.report_identifier == "report-data-run-1-1000"
    assert artifact.generated_at == 1000
    assert pdf_output_path.exists()

    extracted_text = " ".join(
        page.extract_text() or "" for page in PdfReader(pdf_output_path).pages
    )
    assert "Miro Fish Report" in extracted_text
    assert "Executive Summary" in extracted_text
    assert "Demand proof remains thin" in extracted_text
    assert "Transcript Appendix" in extracted_text
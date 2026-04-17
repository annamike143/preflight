use serde::{Deserialize, Serialize};

pub const REPORT_OUTPUT_CONTRACT_NAME: &str = "ReportOutputContract";
pub const REPORT_OUTPUT_CONTRACT_COMMAND_NAME: &str = "ReportOutputContractState";
pub const REPORT_OUTPUT_FILE_FORMAT_NAME: &str = "pdf";
pub const REQUIRED_REPORT_SECTIONS: [ReportSectionName; 7] = [
    ReportSectionName::ExecutiveSummary,
    ReportSectionName::MarketViabilityScore,
    ReportSectionName::RankedRisks,
    ReportSectionName::StrongestProArguments,
    ReportSectionName::StrongestAntiArguments,
    ReportSectionName::CostAndTokenSummary,
    ReportSectionName::TranscriptAppendix,
];
pub const REPORT_OUTPUT_CLASSES: [ReportOutputClass; 6] = [
    ReportOutputClass::ReportMetadata,
    ReportOutputClass::ReportSectionContent,
    ReportOutputClass::TranscriptAppendix,
    ReportOutputClass::GeneratedPdfArtifact,
    ReportOutputClass::ExportAccess,
    ReportOutputClass::ReportAvailability,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportOutputContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportSectionName {
    ExecutiveSummary,
    MarketViabilityScore,
    RankedRisks,
    StrongestProArguments,
    StrongestAntiArguments,
    CostAndTokenSummary,
    TranscriptAppendix,
}

impl ReportSectionName {
    pub fn name(self) -> &'static str {
        match self {
            Self::ExecutiveSummary => "ExecutiveSummary",
            Self::MarketViabilityScore => "MarketViabilityScore",
            Self::RankedRisks => "RankedRisks",
            Self::StrongestProArguments => "StrongestProArguments",
            Self::StrongestAntiArguments => "StrongestAntiArguments",
            Self::CostAndTokenSummary => "CostAndTokenSummary",
            Self::TranscriptAppendix => "TranscriptAppendix",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportOutputClass {
    ReportMetadata,
    ReportSectionContent,
    TranscriptAppendix,
    GeneratedPdfArtifact,
    ExportAccess,
    ReportAvailability,
}

impl ReportOutputClass {
    pub fn name(self) -> &'static str {
        match self {
            Self::ReportMetadata => "ReportMetadata",
            Self::ReportSectionContent => "ReportSectionContent",
            Self::TranscriptAppendix => "TranscriptAppendix",
            Self::GeneratedPdfArtifact => "GeneratedPdfArtifact",
            Self::ExportAccess => "ExportAccess",
            Self::ReportAvailability => "ReportAvailability",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportMetadataContract {
    pub report_identifier_required_when_generated: bool,
    pub run_identifier_required: bool,
    pub session_identifier_required: bool,
    pub completion_classification_required: bool,
    pub output_class_required: bool,
    pub local_artifact_reference_optional_until_generated: bool,
    pub generation_timestamp_optional_until_generated: bool,
    pub file_size_bytes_optional_until_generated: bool,
    pub secret_material_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportSectionRepresentationContract {
    pub executive_summary_defined: bool,
    pub market_viability_score_defined: bool,
    pub ranked_risks_defined: bool,
    pub strongest_pro_arguments_defined: bool,
    pub strongest_anti_arguments_defined: bool,
    pub cost_and_token_summary_defined: bool,
    pub transcript_appendix_defined: bool,
    pub scoring_philosophy_rewritten: bool,
    pub invented_section_variants_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportPdfContract {
    pub file_format_name: &'static str,
    pub input_structure_defined: bool,
    pub output_artifact_class_defined: bool,
    pub successful_generation_implied: bool,
    pub alternate_formats_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportOutputContractSnapshot {
    pub stage: ReportOutputContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub required_sections: [ReportSectionName; 7],
    pub output_classes: [ReportOutputClass; 6],
    pub metadata_contract: ReportMetadataContract,
    pub section_representation_contract: ReportSectionRepresentationContract,
    pub pdf_contract: ReportPdfContract,
    pub stable_output_structure_defined: bool,
    pub real_section_synthesis_materialized: bool,
    pub real_viability_score_materialized: bool,
    pub real_cost_token_summary_materialized: bool,
    pub real_transcript_appendix_materialized: bool,
}

impl ReportOutputContractSnapshot {
    pub fn summary(self) -> String {
        let required_sections = self.required_sections.map(ReportSectionName::name).join(", ");
        let output_classes = self.output_classes.map(ReportOutputClass::name).join(", ");

        format!(
            "Report output contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; required sections are {}; output classes are {}; report identifier required when generated is {}; run identifier required is {}; session identifier required is {}; completion classification required is {}; output class required is {}; local artifact reference optional until generated is {}; generation timestamp optional until generated is {}; file-size-bytes optional until generated is {}; secret material allowed is {}; executive summary defined is {}; market viability score defined is {}; ranked risks defined is {}; strongest pro arguments defined is {}; strongest anti arguments defined is {}; cost and token summary defined is {}; transcript appendix defined is {}; scoring philosophy rewritten is {}; invented section variants allowed is {}; PDF file format is {}; PDF input structure defined is {}; PDF output artifact class defined is {}; successful generation implied is {}; alternate formats allowed is {}; stable output structure defined is {}; real section synthesis materialized is {}; real viability score materialized is {}; real cost and token summary materialized is {}; real transcript appendix materialized is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            required_sections,
            output_classes,
            self.metadata_contract.report_identifier_required_when_generated,
            self.metadata_contract.run_identifier_required,
            self.metadata_contract.session_identifier_required,
            self.metadata_contract.completion_classification_required,
            self.metadata_contract.output_class_required,
            self.metadata_contract.local_artifact_reference_optional_until_generated,
            self.metadata_contract.generation_timestamp_optional_until_generated,
            self.metadata_contract.file_size_bytes_optional_until_generated,
            self.metadata_contract.secret_material_allowed,
            self.section_representation_contract.executive_summary_defined,
            self.section_representation_contract.market_viability_score_defined,
            self.section_representation_contract.ranked_risks_defined,
            self.section_representation_contract.strongest_pro_arguments_defined,
            self.section_representation_contract.strongest_anti_arguments_defined,
            self.section_representation_contract.cost_and_token_summary_defined,
            self.section_representation_contract.transcript_appendix_defined,
            self.section_representation_contract.scoring_philosophy_rewritten,
            self.section_representation_contract.invented_section_variants_allowed,
            self.pdf_contract.file_format_name,
            self.pdf_contract.input_structure_defined,
            self.pdf_contract.output_artifact_class_defined,
            self.pdf_contract.successful_generation_implied,
            self.pdf_contract.alternate_formats_allowed,
            self.stable_output_structure_defined,
            self.real_section_synthesis_materialized,
            self.real_viability_score_materialized,
            self.real_cost_token_summary_materialized,
            self.real_transcript_appendix_materialized,
        )
    }
}

pub fn report_output_contract_state() -> ReportOutputContractSnapshot {
    ReportOutputContractSnapshot {
        stage: ReportOutputContractStage::FoundationMaterialized,
        contract_name: REPORT_OUTPUT_CONTRACT_NAME,
        shell_command_name: REPORT_OUTPUT_CONTRACT_COMMAND_NAME,
        required_sections: REQUIRED_REPORT_SECTIONS,
        output_classes: REPORT_OUTPUT_CLASSES,
        metadata_contract: ReportMetadataContract {
            report_identifier_required_when_generated: true,
            run_identifier_required: true,
            session_identifier_required: true,
            completion_classification_required: true,
            output_class_required: true,
            local_artifact_reference_optional_until_generated: true,
            generation_timestamp_optional_until_generated: true,
            file_size_bytes_optional_until_generated: true,
            secret_material_allowed: false,
        },
        section_representation_contract: ReportSectionRepresentationContract {
            executive_summary_defined: true,
            market_viability_score_defined: true,
            ranked_risks_defined: true,
            strongest_pro_arguments_defined: true,
            strongest_anti_arguments_defined: true,
            cost_and_token_summary_defined: true,
            transcript_appendix_defined: true,
            scoring_philosophy_rewritten: false,
            invented_section_variants_allowed: false,
        },
        pdf_contract: ReportPdfContract {
            file_format_name: REPORT_OUTPUT_FILE_FORMAT_NAME,
            input_structure_defined: true,
            output_artifact_class_defined: true,
            successful_generation_implied: true,
            alternate_formats_allowed: false,
        },
        stable_output_structure_defined: true,
        real_section_synthesis_materialized: true,
        real_viability_score_materialized: true,
        real_cost_token_summary_materialized: true,
        real_transcript_appendix_materialized: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_output_contract_keeps_required_sections_and_output_classes_explicit() {
        let snapshot = report_output_contract_state();

        assert_eq!(snapshot.contract_name, REPORT_OUTPUT_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, REPORT_OUTPUT_CONTRACT_COMMAND_NAME);
        assert_eq!(snapshot.required_sections.len(), 7);
        assert_eq!(snapshot.output_classes.len(), 6);
        assert!(snapshot.metadata_contract.report_identifier_required_when_generated);
        assert!(snapshot.metadata_contract.run_identifier_required);
        assert!(snapshot.metadata_contract.session_identifier_required);
        assert!(snapshot.metadata_contract.completion_classification_required);
        assert!(snapshot.metadata_contract.output_class_required);
        assert!(!snapshot.metadata_contract.secret_material_allowed);
        assert!(snapshot.section_representation_contract.executive_summary_defined);
        assert!(snapshot.section_representation_contract.market_viability_score_defined);
        assert!(snapshot.section_representation_contract.ranked_risks_defined);
        assert!(snapshot.section_representation_contract.strongest_pro_arguments_defined);
        assert!(snapshot.section_representation_contract.strongest_anti_arguments_defined);
        assert!(snapshot.section_representation_contract.cost_and_token_summary_defined);
        assert!(snapshot.section_representation_contract.transcript_appendix_defined);
        assert!(!snapshot.section_representation_contract.scoring_philosophy_rewritten);
        assert!(!snapshot.section_representation_contract.invented_section_variants_allowed);
        assert_eq!(snapshot.pdf_contract.file_format_name, REPORT_OUTPUT_FILE_FORMAT_NAME);
        assert!(snapshot.pdf_contract.input_structure_defined);
        assert!(snapshot.pdf_contract.output_artifact_class_defined);
        assert!(snapshot.pdf_contract.successful_generation_implied);
        assert!(!snapshot.pdf_contract.alternate_formats_allowed);
        assert!(snapshot.stable_output_structure_defined);
        assert!(snapshot.real_section_synthesis_materialized);
        assert!(snapshot.real_viability_score_materialized);
        assert!(snapshot.real_cost_token_summary_materialized);
        assert!(snapshot.real_transcript_appendix_materialized);
    }
}
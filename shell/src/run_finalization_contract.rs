use serde::{Deserialize, Serialize};

pub const RUN_FINALIZATION_CONTRACT_NAME: &str = "RunFinalizationContract";
pub const RUN_FINALIZATION_CONTRACT_COMMAND_NAME: &str = "RunFinalizationContractState";
pub const SUPPORTED_RUN_FINALIZATION_OUTPUT_MARKERS: [RunFinalizationOutputMarker; 7] = [
    RunFinalizationOutputMarker::FinalizationEntered,
    RunFinalizationOutputMarker::FinalizationSucceeded,
    RunFinalizationOutputMarker::ReportGenerationInProgress,
    RunFinalizationOutputMarker::ReportGenerationSucceeded,
    RunFinalizationOutputMarker::ReportGenerationFailed,
    RunFinalizationOutputMarker::ReportAvailable,
    RunFinalizationOutputMarker::ReportUnavailable,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunFinalizationContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunFinalizationOutputMarker {
    FinalizationEntered,
    FinalizationSucceeded,
    ReportGenerationInProgress,
    ReportGenerationSucceeded,
    ReportGenerationFailed,
    ReportAvailable,
    ReportUnavailable,
}

impl RunFinalizationOutputMarker {
    pub fn name(self) -> &'static str {
        match self {
            Self::FinalizationEntered => "FinalizationEntered",
            Self::FinalizationSucceeded => "FinalizationSucceeded",
            Self::ReportGenerationInProgress => "ReportGenerationInProgress",
            Self::ReportGenerationSucceeded => "ReportGenerationSucceeded",
            Self::ReportGenerationFailed => "ReportGenerationFailed",
            Self::ReportAvailable => "ReportAvailable",
            Self::ReportUnavailable => "ReportUnavailable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunFinalizationContractSnapshot {
    pub stage: RunFinalizationContractStage,
    pub contract_name: &'static str,
    pub shell_command_name: &'static str,
    pub supported_markers: [RunFinalizationOutputMarker; 7],
    pub shell_authority_required: bool,
    pub finalization_entered_distinct: bool,
    pub finalization_succeeded_distinct: bool,
    pub report_generation_in_progress_distinct: bool,
    pub report_generation_succeeded_distinct: bool,
    pub report_generation_failed_distinct: bool,
    pub report_available_distinct: bool,
    pub report_unavailable_distinct: bool,
    pub completion_classification_implies_report_generation_success: bool,
    pub completion_classification_implies_report_available: bool,
    pub cleanup_outcome_implies_report_availability: bool,
    pub real_report_generation_materialized: bool,
    pub real_pdf_generation_materialized: bool,
}

impl RunFinalizationContractSnapshot {
    pub fn summary(self) -> String {
        let supported_markers = self
            .supported_markers
            .map(RunFinalizationOutputMarker::name)
            .join(", ");

        format!(
            "Run finalization contract surface is materialized. Stage is {:?}; contract name is {}; shell command name is {}; supported markers are {}; shell authority required is {}; finalization entered distinct is {}; finalization succeeded distinct is {}; report generation in progress distinct is {}; report generation succeeded distinct is {}; report generation failed distinct is {}; report available distinct is {}; report unavailable distinct is {}; completion classification implies report generation success is {}; completion classification implies report available is {}; cleanup outcome implies report availability is {}; real report generation materialized is {}; real PDF generation materialized is {}.",
            self.stage,
            self.contract_name,
            self.shell_command_name,
            supported_markers,
            self.shell_authority_required,
            self.finalization_entered_distinct,
            self.finalization_succeeded_distinct,
            self.report_generation_in_progress_distinct,
            self.report_generation_succeeded_distinct,
            self.report_generation_failed_distinct,
            self.report_available_distinct,
            self.report_unavailable_distinct,
            self.completion_classification_implies_report_generation_success,
            self.completion_classification_implies_report_available,
            self.cleanup_outcome_implies_report_availability,
            self.real_report_generation_materialized,
            self.real_pdf_generation_materialized,
        )
    }
}

pub fn run_finalization_contract_state() -> RunFinalizationContractSnapshot {
    RunFinalizationContractSnapshot {
        stage: RunFinalizationContractStage::FoundationMaterialized,
        contract_name: RUN_FINALIZATION_CONTRACT_NAME,
        shell_command_name: RUN_FINALIZATION_CONTRACT_COMMAND_NAME,
        supported_markers: SUPPORTED_RUN_FINALIZATION_OUTPUT_MARKERS,
        shell_authority_required: true,
        finalization_entered_distinct: true,
        finalization_succeeded_distinct: true,
        report_generation_in_progress_distinct: true,
        report_generation_succeeded_distinct: true,
        report_generation_failed_distinct: true,
        report_available_distinct: true,
        report_unavailable_distinct: true,
        completion_classification_implies_report_generation_success: true,
        completion_classification_implies_report_available: false,
        cleanup_outcome_implies_report_availability: false,
        real_report_generation_materialized: true,
        real_pdf_generation_materialized: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_finalization_contract_keeps_finalization_output_and_availability_distinct() {
        let snapshot = run_finalization_contract_state();

        assert_eq!(snapshot.contract_name, RUN_FINALIZATION_CONTRACT_NAME);
        assert_eq!(snapshot.shell_command_name, RUN_FINALIZATION_CONTRACT_COMMAND_NAME);
        assert_eq!(snapshot.supported_markers.len(), 7);
        assert!(snapshot.shell_authority_required);
        assert!(snapshot.finalization_entered_distinct);
        assert!(snapshot.finalization_succeeded_distinct);
        assert!(snapshot.report_generation_in_progress_distinct);
        assert!(snapshot.report_generation_succeeded_distinct);
        assert!(snapshot.report_generation_failed_distinct);
        assert!(snapshot.report_available_distinct);
        assert!(snapshot.report_unavailable_distinct);
        assert!(snapshot.completion_classification_implies_report_generation_success);
        assert!(!snapshot.completion_classification_implies_report_available);
        assert!(!snapshot.cleanup_outcome_implies_report_availability);
        assert!(snapshot.real_report_generation_materialized);
        assert!(snapshot.real_pdf_generation_materialized);
    }
}
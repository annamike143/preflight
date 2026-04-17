use serde::{Deserialize, Serialize};

pub const RUN_TOKEN_DURATION_TRACKING_NAME: &str = "RunTokenDurationTracking";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunTokenDurationTrackingStage {
    ExecutionMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunTokenDurationTrackingSnapshot {
    pub stage: RunTokenDurationTrackingStage,
    pub surface_name: &'static str,
    pub engine_owned_tracking: bool,
    pub run_local_tracking_required: bool,
    pub turn_level_tracking_required: bool,
    pub duration_tracking_required: bool,
    pub integrated_into_execution_path: bool,
    pub provider_billing_equivalence_implied: bool,
    pub live_monitoring_implied: bool,
    pub final_report_implied: bool,
    pub token_duration_tracking_materialized: bool,
}

impl RunTokenDurationTrackingSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Run token-and-duration tracking surface is materialized. Stage is {:?}; surface name is {}; engine-owned tracking is {}; run-local tracking required is {}; turn-level tracking required is {}; duration tracking required is {}; integrated into execution path is {}; provider-billing equivalence implied is {}; live monitoring implied is {}; final report implied is {}; token-and-duration tracking materialized is {}.",
            self.stage,
            self.surface_name,
            self.engine_owned_tracking,
            self.run_local_tracking_required,
            self.turn_level_tracking_required,
            self.duration_tracking_required,
            self.integrated_into_execution_path,
            self.provider_billing_equivalence_implied,
            self.live_monitoring_implied,
            self.final_report_implied,
            self.token_duration_tracking_materialized,
        )
    }
}

pub fn run_token_duration_tracking_surface() -> RunTokenDurationTrackingSnapshot {
    RunTokenDurationTrackingSnapshot {
        stage: RunTokenDurationTrackingStage::ExecutionMaterialized,
        surface_name: RUN_TOKEN_DURATION_TRACKING_NAME,
        engine_owned_tracking: true,
        run_local_tracking_required: true,
        turn_level_tracking_required: true,
        duration_tracking_required: true,
        integrated_into_execution_path: true,
        provider_billing_equivalence_implied: false,
        live_monitoring_implied: false,
        final_report_implied: false,
        token_duration_tracking_materialized: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_token_duration_tracking_surface_keeps_tracking_local_and_non_billing() {
        let snapshot = run_token_duration_tracking_surface();

        assert_eq!(snapshot.surface_name, RUN_TOKEN_DURATION_TRACKING_NAME);
        assert!(snapshot.engine_owned_tracking);
        assert!(snapshot.run_local_tracking_required);
        assert!(snapshot.turn_level_tracking_required);
        assert!(snapshot.duration_tracking_required);
        assert!(snapshot.integrated_into_execution_path);
        assert!(!snapshot.provider_billing_equivalence_implied);
        assert!(!snapshot.live_monitoring_implied);
        assert!(!snapshot.final_report_implied);
        assert!(snapshot.token_duration_tracking_materialized);
    }
}
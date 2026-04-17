use serde::{Deserialize, Serialize};

pub const RUN_MODERATED_ROUNDS_NAME: &str = "RunModeratedRounds";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunModeratedRoundsStage {
    ExecutionMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunModeratedRoundsSnapshot {
    pub stage: RunModeratedRoundsStage,
    pub surface_name: &'static str,
    pub engine_owned_moderation: bool,
    pub moderator_controlled_selection_required: bool,
    pub normal_round_min_speakers: u8,
    pub normal_round_max_speakers: u8,
    pub all_agent_fanout_allowed: bool,
    pub speaker_attribution_required: bool,
    pub round_summary_continuity_required: bool,
    pub argument_risk_continuity_required: bool,
    pub moderated_round_execution_materialized: bool,
    pub live_monitoring_implied: bool,
    pub final_report_implied: bool,
}

impl RunModeratedRoundsSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Run moderated-rounds surface is materialized. Stage is {:?}; surface name is {}; engine-owned moderation is {}; Moderator-controlled speaker selection required is {}; normal-round minimum speakers is {}; normal-round maximum speakers is {}; all-agent fanout allowed is {}; speaker attribution required is {}; round-summary continuity required is {}; argument-and-risk continuity required is {}; moderated-round execution materialized is {}; live monitoring implied is {}; final report implied is {}.",
            self.stage,
            self.surface_name,
            self.engine_owned_moderation,
            self.moderator_controlled_selection_required,
            self.normal_round_min_speakers,
            self.normal_round_max_speakers,
            self.all_agent_fanout_allowed,
            self.speaker_attribution_required,
            self.round_summary_continuity_required,
            self.argument_risk_continuity_required,
            self.moderated_round_execution_materialized,
            self.live_monitoring_implied,
            self.final_report_implied,
        )
    }
}

pub fn run_moderated_rounds_surface() -> RunModeratedRoundsSnapshot {
    RunModeratedRoundsSnapshot {
        stage: RunModeratedRoundsStage::ExecutionMaterialized,
        surface_name: RUN_MODERATED_ROUNDS_NAME,
        engine_owned_moderation: true,
        moderator_controlled_selection_required: true,
        normal_round_min_speakers: 3,
        normal_round_max_speakers: 5,
        all_agent_fanout_allowed: false,
        speaker_attribution_required: true,
        round_summary_continuity_required: true,
        argument_risk_continuity_required: true,
        moderated_round_execution_materialized: true,
        live_monitoring_implied: false,
        final_report_implied: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_moderated_rounds_surface_keeps_execution_bounded_and_non_streaming() {
        let snapshot = run_moderated_rounds_surface();

        assert_eq!(snapshot.surface_name, RUN_MODERATED_ROUNDS_NAME);
        assert!(snapshot.engine_owned_moderation);
        assert!(snapshot.moderator_controlled_selection_required);
        assert_eq!(snapshot.normal_round_min_speakers, 3);
        assert_eq!(snapshot.normal_round_max_speakers, 5);
        assert!(!snapshot.all_agent_fanout_allowed);
        assert!(snapshot.speaker_attribution_required);
        assert!(snapshot.round_summary_continuity_required);
        assert!(snapshot.argument_risk_continuity_required);
        assert!(snapshot.moderated_round_execution_materialized);
        assert!(!snapshot.live_monitoring_implied);
        assert!(!snapshot.final_report_implied);
    }
}
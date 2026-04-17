use serde::{Deserialize, Serialize};

pub const RUN_BOUNDED_MEMORY_NAME: &str = "RunBoundedMemory";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunBoundedMemoryStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunBoundedMemorySnapshot {
    pub stage: RunBoundedMemoryStage,
    pub memory_surface_name: &'static str,
    pub seed_grounding_memory_required: bool,
    pub round_summary_memory_required: bool,
    pub argument_risk_memory_required: bool,
    pub transcript_archive_separate_from_prompt_memory: bool,
    pub naive_full_transcript_replay_allowed: bool,
    pub cross_run_semantic_leakage_allowed: bool,
    pub bounded_memory_foundation_materialized: bool,
    pub transcript_archive_foundation_materialized: bool,
    pub final_report_implied: bool,
}

impl RunBoundedMemorySnapshot {
    pub fn summary(self) -> String {
        format!(
            "Run bounded-memory surface is materialized. Stage is {:?}; memory surface name is {}; seed-grounding memory required is {}; round-summary memory required is {}; argument-or-risk memory required is {}; transcript archive separate from prompt memory is {}; naive full-transcript replay allowed is {}; cross-run semantic leakage allowed is {}; bounded-memory foundation materialized is {}; transcript-archive foundation materialized is {}; final report implied is {}.",
            self.stage,
            self.memory_surface_name,
            self.seed_grounding_memory_required,
            self.round_summary_memory_required,
            self.argument_risk_memory_required,
            self.transcript_archive_separate_from_prompt_memory,
            self.naive_full_transcript_replay_allowed,
            self.cross_run_semantic_leakage_allowed,
            self.bounded_memory_foundation_materialized,
            self.transcript_archive_foundation_materialized,
            self.final_report_implied,
        )
    }
}

pub fn run_bounded_memory_surface() -> RunBoundedMemorySnapshot {
    RunBoundedMemorySnapshot {
        stage: RunBoundedMemoryStage::FoundationMaterialized,
        memory_surface_name: RUN_BOUNDED_MEMORY_NAME,
        seed_grounding_memory_required: true,
        round_summary_memory_required: true,
        argument_risk_memory_required: true,
        transcript_archive_separate_from_prompt_memory: true,
        naive_full_transcript_replay_allowed: false,
        cross_run_semantic_leakage_allowed: false,
        bounded_memory_foundation_materialized: true,
        transcript_archive_foundation_materialized: true,
        final_report_implied: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_bounded_memory_surface_forbids_transcript_replay_and_cross_run_leakage() {
        let snapshot = run_bounded_memory_surface();

        assert_eq!(snapshot.memory_surface_name, RUN_BOUNDED_MEMORY_NAME);
        assert!(snapshot.seed_grounding_memory_required);
        assert!(snapshot.round_summary_memory_required);
        assert!(snapshot.argument_risk_memory_required);
        assert!(snapshot.transcript_archive_separate_from_prompt_memory);
        assert!(!snapshot.naive_full_transcript_replay_allowed);
        assert!(!snapshot.cross_run_semantic_leakage_allowed);
        assert!(snapshot.bounded_memory_foundation_materialized);
        assert!(snapshot.transcript_archive_foundation_materialized);
        assert!(!snapshot.final_report_implied);
    }
}
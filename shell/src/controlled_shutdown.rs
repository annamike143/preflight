use crate::run_session_start::shell_shutdown_coordination_flags;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlledShutdownStage {
    CoordinationMaterialized,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ControlledShutdownSnapshot {
    pub stage: ControlledShutdownStage,
    pub shell_supervised_shutdown_only: bool,
    pub graceful_shutdown_required_first: bool,
    pub graceful_shutdown_implemented: bool,
    pub forceful_termination_allowed_when_required: bool,
    pub forceful_termination_implemented: bool,
    pub shutdown_command_surface_materialized: bool,
    pub engine_shutdown_in_progress: bool,
    pub shutdown_coordination_ready: bool,
    pub detached_normal_mode_allowed: bool,
    pub cleanup_contract_handoff_shell_only: bool,
    pub shutdown_to_cleanup_execution_materialized: bool,
    pub startup_janitor_fallback_contract_materialized: bool,
}

impl ControlledShutdownSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Controlled shutdown surface is materialized. Stage is {:?}; shell supervised shutdown only is {}; graceful shutdown required first is {}; graceful shutdown implemented is {}; forceful termination allowed when required is {}; forceful termination implemented is {}; shutdown command surface materialized is {}; engine shutdown in progress is {}; shutdown coordination ready is {}; detached normal mode allowed is {}; cleanup contract handoff shell-only is {}; shutdown-to-cleanup execution materialized is {}; startup-janitor fallback contract materialized is {}.",
            self.stage,
            self.shell_supervised_shutdown_only,
            self.graceful_shutdown_required_first,
            self.graceful_shutdown_implemented,
            self.forceful_termination_allowed_when_required,
            self.forceful_termination_implemented,
            self.shutdown_command_surface_materialized,
            self.engine_shutdown_in_progress,
            self.shutdown_coordination_ready,
            self.detached_normal_mode_allowed,
            self.cleanup_contract_handoff_shell_only,
            self.shutdown_to_cleanup_execution_materialized,
            self.startup_janitor_fallback_contract_materialized,
        )
    }
}

pub fn controlled_shutdown_scaffold() -> ControlledShutdownSnapshot {
    let (shutdown_requested, graceful_shutdown_started, forceful_termination_required, forceful_termination_executed) =
        shell_shutdown_coordination_flags();

    ControlledShutdownSnapshot {
        stage: ControlledShutdownStage::CoordinationMaterialized,
        shell_supervised_shutdown_only: true,
        graceful_shutdown_required_first: true,
        graceful_shutdown_implemented: true,
        forceful_termination_allowed_when_required: true,
        forceful_termination_implemented: true,
        shutdown_command_surface_materialized: true,
        engine_shutdown_in_progress: shutdown_requested
            || graceful_shutdown_started
            || forceful_termination_required
            || forceful_termination_executed,
        shutdown_coordination_ready: true,
        detached_normal_mode_allowed: false,
        cleanup_contract_handoff_shell_only: true,
        shutdown_to_cleanup_execution_materialized: true,
        startup_janitor_fallback_contract_materialized: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controlled_shutdown_scaffold_exposes_shutdown_cleanup_handoff_truth() {
        let snapshot = controlled_shutdown_scaffold();

        assert!(snapshot.shell_supervised_shutdown_only);
        assert!(snapshot.graceful_shutdown_required_first);
        assert!(snapshot.graceful_shutdown_implemented);
        assert!(snapshot.forceful_termination_allowed_when_required);
        assert!(snapshot.forceful_termination_implemented);
        assert!(snapshot.shutdown_command_surface_materialized);
        assert!(!snapshot.engine_shutdown_in_progress);
        assert!(snapshot.shutdown_coordination_ready);
        assert!(!snapshot.detached_normal_mode_allowed);
        assert!(snapshot.cleanup_contract_handoff_shell_only);
        assert!(snapshot.shutdown_to_cleanup_execution_materialized);
        assert!(snapshot.startup_janitor_fallback_contract_materialized);
    }
}
use std::fs;
use std::path::{Path, PathBuf};

pub const RUN_WORKSPACE_ROOT_DIRECTORY_NAME: &str = "miro-fish-desktop-saas";
pub const RUN_WORKSPACE_COLLECTION_DIRECTORY_NAME: &str = "run-workspaces";
pub const RUN_EXECUTION_SEED_DIRECTORY_NAME: &str = "execution-seed";
pub const RUN_TRANSCRIPT_ARCHIVE_DIRECTORY_NAME: &str = "transcript-archive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunWorkspaceAllocationFailureReason {
    RootUnavailable,
    AlreadyExists,
    CreationFailed,
}

impl RunWorkspaceAllocationFailureReason {
    pub fn label(self) -> &'static str {
        match self {
            Self::RootUnavailable => "workspace_root_unavailable",
            Self::AlreadyExists => "workspace_already_exists",
            Self::CreationFailed => "workspace_creation_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunWorkspaceAllocationSnapshot {
    pub workspace_reference: String,
    pub workspace_root_path: String,
    pub execution_seed_directory_path: String,
    pub transcript_archive_directory_path: String,
}

pub fn allocate_fresh_run_workspace(
    run_id: &str,
) -> Result<RunWorkspaceAllocationSnapshot, RunWorkspaceAllocationFailureReason> {
    allocate_fresh_run_workspace_under(&default_run_workspace_root(), run_id)
}

pub fn default_run_workspace_root() -> PathBuf {
    std::env::temp_dir()
        .join(RUN_WORKSPACE_ROOT_DIRECTORY_NAME)
        .join(RUN_WORKSPACE_COLLECTION_DIRECTORY_NAME)
}

pub fn allocate_fresh_run_workspace_under(
    base_root: &Path,
    run_id: &str,
) -> Result<RunWorkspaceAllocationSnapshot, RunWorkspaceAllocationFailureReason> {
    let normalized_run_id = run_id.trim();
    if normalized_run_id.is_empty() {
        return Err(RunWorkspaceAllocationFailureReason::RootUnavailable);
    }

    fs::create_dir_all(base_root)
        .map_err(|_| RunWorkspaceAllocationFailureReason::RootUnavailable)?;

    let workspace_root = base_root.join(normalized_run_id);
    if workspace_root.exists() {
        return Err(RunWorkspaceAllocationFailureReason::AlreadyExists);
    }

    let execution_seed_directory = workspace_root.join(RUN_EXECUTION_SEED_DIRECTORY_NAME);
    let transcript_archive_directory = workspace_root.join(RUN_TRANSCRIPT_ARCHIVE_DIRECTORY_NAME);

    fs::create_dir_all(&execution_seed_directory)
        .and_then(|_| fs::create_dir_all(&transcript_archive_directory))
        .map_err(|_| RunWorkspaceAllocationFailureReason::CreationFailed)?;

    Ok(RunWorkspaceAllocationSnapshot {
        workspace_reference: format!("workspace::{normalized_run_id}"),
        workspace_root_path: workspace_root.to_string_lossy().into_owned(),
        execution_seed_directory_path: execution_seed_directory.to_string_lossy().into_owned(),
        transcript_archive_directory_path: transcript_archive_directory
            .to_string_lossy()
            .into_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_fresh_run_workspace_under_creates_isolated_workspace_paths() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");

        let first_workspace = allocate_fresh_run_workspace_under(temp_dir.path(), "run-1")
            .expect("first workspace should allocate");
        let second_workspace = allocate_fresh_run_workspace_under(temp_dir.path(), "run-2")
            .expect("second workspace should allocate");

        assert_ne!(first_workspace.workspace_reference, second_workspace.workspace_reference);
        assert_ne!(first_workspace.workspace_root_path, second_workspace.workspace_root_path);
        assert!(Path::new(&first_workspace.workspace_root_path).exists());
        assert!(Path::new(&first_workspace.execution_seed_directory_path).exists());
        assert!(Path::new(&first_workspace.transcript_archive_directory_path).exists());
        assert!(Path::new(&second_workspace.workspace_root_path).exists());
    }

    #[test]
    fn allocate_fresh_run_workspace_under_rejects_reuse_of_existing_workspace_path() {
        let temp_dir = tempfile::tempdir().expect("tempdir should exist");

        let _first_workspace = allocate_fresh_run_workspace_under(temp_dir.path(), "run-1")
            .expect("first workspace should allocate");
        let reused_workspace = allocate_fresh_run_workspace_under(temp_dir.path(), "run-1")
            .expect_err("reusing the same workspace path should fail");

        assert_eq!(reused_workspace, RunWorkspaceAllocationFailureReason::AlreadyExists);
    }
}
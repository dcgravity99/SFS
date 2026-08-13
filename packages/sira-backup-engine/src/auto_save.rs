/* ============================================================================
 * Siragugal Film Studio — Module 36: Project Backup & Auto-Save Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupSnapshotSpec {
    pub snapshot_id: String,
    pub project_path: String,
    pub snapshot_reason: String,
    pub created_at_utc: String,
}

#[derive(Default)]
pub struct AutoSaveBackupEngine {
    snapshots: Vec<BackupSnapshotSpec>,
}

impl AutoSaveBackupEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger_snapshot(&mut self, spec: &BackupSnapshotSpec) -> SiraResult<String> {
        if spec.project_path.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_BACKUP_PATH".to_string(),
                category: "BACKUP_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.backup.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }
        self.snapshots.push(spec.clone());
        SiraResult::Success(spec.snapshot_id.clone())
    }

    pub fn list_snapshots(&self, project_path: &str) -> SiraResult<Vec<BackupSnapshotSpec>> {
        let matched: Vec<BackupSnapshotSpec> = self
            .snapshots
            .iter()
            .filter(|s| s.project_path == project_path)
            .cloned()
            .collect();
        SiraResult::Success(matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_36_auto_save_lifecycle() {
        let mut engine = AutoSaveBackupEngine::new();
        let spec = BackupSnapshotSpec {
            snapshot_id: "SNAP-2026-001".to_string(),
            project_path: "C:/Projects/FeatureFilm.sfsp".to_string(),
            snapshot_reason: "AutoSave".to_string(),
            created_at_utc: "2026-08-13T21:15:00Z".to_string(),
        };

        let res = engine.trigger_snapshot(&spec);
        assert!(matches!(res, SiraResult::Success(_)));

        let list_res = engine.list_snapshots("C:/Projects/FeatureFilm.sfsp");
        if let SiraResult::Success(list) = list_res {
            assert_eq!(list.len(), 1);
            assert_eq!(list[0].snapshot_id, "SNAP-2026-001");
        }

        // Test path traversal rejection
        let invalid_spec = BackupSnapshotSpec {
            snapshot_id: "SNAP-INVALID".to_string(),
            project_path: "C:/Projects/../Traversed".to_string(),
            snapshot_reason: "AutoSave".to_string(),
            created_at_utc: "2026-08-13T21:15:00Z".to_string(),
        };
        assert!(matches!(engine.trigger_snapshot(&invalid_spec), SiraResult::Error(_)));
    }
}

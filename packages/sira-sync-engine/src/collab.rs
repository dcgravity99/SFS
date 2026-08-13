/* ============================================================================
 * Siragugal Film Studio — Module 33: Multi-User Real-time Collaborative Editing Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollabSessionConfig {
    pub session_id: String,
    pub project_id: String,
    pub user_id: String,
    pub user_role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineDeltaOp {
    pub op_id: String,
    pub target_track_id: String,
    pub operation_type: String,
    pub payload_json: String,
}

#[derive(Default)]
pub struct CollabSyncEngine {
    sessions: Vec<CollabSessionConfig>,
    delta_log: Vec<TimelineDeltaOp>,
}

impl CollabSyncEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn join_session(&mut self, config: &CollabSessionConfig) -> SiraResult<bool> {
        if config.session_id.is_empty() || config.user_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_COLLAB_SESSION_CONFIG".to_string(),
                category: "SYNC_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.collab.invalid_session".to_string(),
                suggested_action_key: None,
            });
        }
        self.sessions.push(config.clone());
        SiraResult::Success(true)
    }

    pub fn submit_delta_op(&mut self, op: &TimelineDeltaOp) -> SiraResult<String> {
        if op.op_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_DELTA_OP_ID".to_string(),
                category: "SYNC_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.collab.empty_op_id".to_string(),
                suggested_action_key: None,
            });
        }
        self.delta_log.push(op.clone());
        SiraResult::Success(op.op_id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_33_collab_sync_lifecycle() {
        let mut engine = CollabSyncEngine::new();
        let config = CollabSessionConfig {
            session_id: "SESS-COLLAB-001".to_string(),
            project_id: "PROJ-001".to_string(),
            user_id: "USER-EDITOR-ALICE".to_string(),
            user_role: "Director".to_string(),
        };

        let join_res = engine.join_session(&config);
        assert!(matches!(join_res, SiraResult::Success(_)));

        let delta = TimelineDeltaOp {
            op_id: "OP-DELTA-100".to_string(),
            target_track_id: "TRACK-V1".to_string(),
            operation_type: "InsertClip".to_string(),
            payload_json: r#"{"clip_id":"CLIP-01"}"#.to_string(),
        };

        let submit_res = engine.submit_delta_op(&delta);
        assert!(matches!(submit_res, SiraResult::Success(_)));

        // Test invalid empty session
        let invalid_config = CollabSessionConfig {
            session_id: "".to_string(),
            project_id: "PROJ-001".to_string(),
            user_id: "".to_string(),
            user_role: "Viewer".to_string(),
        };
        assert!(matches!(engine.join_session(&invalid_config), SiraResult::Error(_)));
    }
}

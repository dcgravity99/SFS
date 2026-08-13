/* ============================================================================
 * Siragugal Film Studio — Module 55: Global Ecosystem Orchestrator & Master Dispatcher Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MasterJobTask {
    pub task_id: String,
    pub target_engine: String,
    pub payload_json: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MasterJobDag {
    pub master_job_id: String,
    pub tasks: Vec<MasterJobTask>,
}

#[derive(Default)]
pub struct MasterDispatcherEngine;

impl MasterDispatcherEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dispatch_master_dag(&self, dag: &MasterJobDag) -> SiraResult<String> {
        if dag.master_job_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_MASTER_JOB_ID".to_string(),
                category: "ECOSYSTEM_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.master_dispatcher.empty_id".to_string(),
                suggested_action_key: None,
            });
        }

        if dag.master_job_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_MASTER_JOB_PATH".to_string(),
                category: "ECOSYSTEM_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.master_dispatcher.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        if dag.tasks.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_MASTER_DAG_TASKS".to_string(),
                category: "ECOSYSTEM_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.master_dispatcher.empty_tasks".to_string(),
                suggested_action_key: None,
            });
        }

        SiraResult::Success(format!("DISPATCHED-MASTER-JOB-{}", dag.master_job_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_55_master_dispatcher_lifecycle() {
        let engine = MasterDispatcherEngine::new();
        let dag = MasterJobDag {
            master_job_id: "JOB-FEATURE-FILM-01".to_string(),
            tasks: vec![MasterJobTask {
                task_id: "TASK-01".to_string(),
                target_engine: "sira_engine_director".to_string(),
                payload_json: r#"{"action":"shot_detection"}"#.to_string(),
            }],
        };

        let dispatch_res = engine.dispatch_master_dag(&dag);
        assert!(matches!(dispatch_res, SiraResult::Success(_)));

        if let SiraResult::Success(job_id) = dispatch_res {
            assert_eq!(job_id, "DISPATCHED-MASTER-JOB-JOB-FEATURE-FILM-01");
        }

        // Test empty job ID rejection
        let invalid_dag = MasterJobDag {
            master_job_id: "".to_string(),
            tasks: vec![],
        };
        assert!(matches!(engine.dispatch_master_dag(&invalid_dag), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_dag = MasterJobDag {
            master_job_id: "JOB/../traversed".to_string(),
            tasks: vec![],
        };
        assert!(matches!(engine.dispatch_master_dag(&path_invalid_dag), SiraResult::Error(_)));
    }
}

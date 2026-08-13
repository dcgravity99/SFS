/* ============================================================================
 * Siragugal Film Studio — Module 32: Automated Render Farm Dispatcher
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderJobSpec {
    pub job_id: String,
    pub project_path: String,
    pub start_frame: u32,
    pub end_frame: u32,
    pub priority: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderNodeStatus {
    pub node_id: String,
    pub is_online: bool,
    pub current_load_percentage: f32,
    pub active_job_id: Option<String>,
}

#[derive(Default)]
pub struct RenderFarmDispatcher {
    jobs: Vec<RenderJobSpec>,
    nodes: Vec<RenderNodeStatus>,
}

impl RenderFarmDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_node(&mut self, node: RenderNodeStatus) {
        self.nodes.push(node);
    }

    pub fn dispatch_job(&mut self, spec: &RenderJobSpec) -> SiraResult<String> {
        if spec.project_path.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_RENDER_PROJECT_PATH".to_string(),
                category: "DEPLOYMENT_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.render_farm.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }
        if spec.end_frame < spec.start_frame {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_FRAME_RANGE".to_string(),
                category: "DEPLOYMENT_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.render_farm.invalid_range".to_string(),
                suggested_action_key: None,
            });
        }
        self.jobs.push(spec.clone());
        SiraResult::Success(spec.job_id.clone())
    }

    pub fn query_node_health(&self) -> SiraResult<Vec<RenderNodeStatus>> {
        SiraResult::Success(self.nodes.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_32_render_farm_dispatcher_lifecycle() {
        let mut dispatcher = RenderFarmDispatcher::new();
        dispatcher.register_node(RenderNodeStatus {
            node_id: "NODE-GPU-01".to_string(),
            is_online: true,
            current_load_percentage: 12.5,
            active_job_id: None,
        });

        let spec = RenderJobSpec {
            job_id: "JOB-RENDER-1001".to_string(),
            project_path: "C:/Projects/Movie.sfsp".to_string(),
            start_frame: 1,
            end_frame: 240,
            priority: 10,
        };

        let dispatch_res = dispatcher.dispatch_job(&spec);
        assert!(matches!(dispatch_res, SiraResult::Success(_)));

        let nodes_res = dispatcher.query_node_health();
        if let SiraResult::Success(nodes) = nodes_res {
            assert_eq!(nodes.len(), 1);
            assert_eq!(nodes[0].node_id, "NODE-GPU-01");
        }

        // Test invalid frame range
        let invalid_range = RenderJobSpec {
            job_id: "JOB-INVALID".to_string(),
            project_path: "C:/Projects/Movie.sfsp".to_string(),
            start_frame: 500,
            end_frame: 100,
            priority: 1,
        };
        assert!(matches!(dispatcher.dispatch_job(&invalid_range), SiraResult::Error(_)));
    }
}

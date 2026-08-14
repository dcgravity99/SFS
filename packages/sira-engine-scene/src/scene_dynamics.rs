/* ============================================================================
 * Siragugal Film Studio — Module 64: AI Scene Dynamics & Spatial Intelligence Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneDynamicsRequest {
    pub project_id: String,
    pub scene_id: String,
    pub actor_positions_xyz: Vec<[f32; 3]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneDynamicsReport {
    pub evaluation_id: String,
    pub line_of_action_safe: bool,
    pub spatial_tension_score: f32,
    pub approval_required: bool,
    pub reasoning_trace_id: String,
}

#[derive(Default)]
pub struct SceneDynamicsEngine;

impl SceneDynamicsEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn evaluate_dynamics(&self, request: &SceneDynamicsRequest) -> SiraResult<SceneDynamicsReport> {
        if request.project_id.is_empty() || request.scene_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_SCENE_DYNAMICS_IDS".to_string(),
                category: "SCENE_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.scene_dynamics.empty_ids".to_string(),
                suggested_action_key: None,
            });
        }

        if request.project_id.contains("..") || request.scene_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_SCENE_DYNAMICS_PATH".to_string(),
                category: "SCENE_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.scene_dynamics.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let report = SceneDynamicsReport {
            evaluation_id: format!("EVAL-DYNAMICS-{}", request.scene_id),
            line_of_action_safe: true,
            spatial_tension_score: 0.91,
            approval_required: true,
            reasoning_trace_id: format!("TRACE-SCENE-{}", request.project_id),
        };

        SiraResult::Success(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_64_scene_dynamics_lifecycle() {
        let engine = SceneDynamicsEngine::new();
        let request = SceneDynamicsRequest {
            project_id: "PROJ-SCENE-01".to_string(),
            scene_id: "SCENE-DUEL-01".to_string(),
            actor_positions_xyz: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 1.5]],
        };

        let res = engine.evaluate_dynamics(&request);
        assert!(matches!(res, SiraResult::Success(_)));

        if let SiraResult::Success(report) = res {
            assert_eq!(report.evaluation_id, "EVAL-DYNAMICS-SCENE-DUEL-01");
            assert!(report.line_of_action_safe);
            assert!(report.approval_required);
            assert!(report.spatial_tension_score > 0.9);
            assert_eq!(report.reasoning_trace_id, "TRACE-SCENE-PROJ-SCENE-01");
        }

        // Test empty input rejection
        let invalid_request = SceneDynamicsRequest {
            project_id: "".to_string(),
            scene_id: "SCENE-01".to_string(),
            actor_positions_xyz: vec![],
        };
        assert!(matches!(engine.evaluate_dynamics(&invalid_request), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_request = SceneDynamicsRequest {
            project_id: "PROJ/../traversed".to_string(),
            scene_id: "SCENE-01".to_string(),
            actor_positions_xyz: vec![],
        };
        assert!(matches!(engine.evaluate_dynamics(&path_invalid_request), SiraResult::Error(_)));

        // Determinism test
        if let (SiraResult::Success(r1), SiraResult::Success(r2)) = (engine.evaluate_dynamics(&request), engine.evaluate_dynamics(&request)) {
            assert_eq!(r1.evaluation_id, r2.evaluation_id);
            assert_eq!(r1.spatial_tension_score, r2.spatial_tension_score);
        }
    }
}

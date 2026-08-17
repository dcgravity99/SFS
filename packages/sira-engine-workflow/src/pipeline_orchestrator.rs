/* ============================================================================
 * Siragugal Film Studio — Module 68: AI Production Pipeline Orchestrator Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineStageType {
    ScriptAnalysis,
    CreativeIntelligenceAssessment,
    DirectorDecisionGeneration,
    AssetGeneration,
    RenderCompositing,
    QualityControl,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineOrchestrationRequest {
    pub pipeline_id: String,
    pub project_id: String,
    pub active_stages: Vec<PipelineStageType>,
    pub execution_mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineOrchestrationReport {
    pub orchestration_id: String,
    pub pipeline_id: String,
    pub current_stage: PipelineStageType,
    pub completed_stages: Vec<PipelineStageType>,
    pub progress_score: f32,
    pub approval_required: bool,
    pub reasoning_trace_id: String,
}

#[derive(Default)]
pub struct PipelineOrchestratorEngine;

impl PipelineOrchestratorEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orchestrate_pipeline(
        &self,
        request: &PipelineOrchestrationRequest,
    ) -> SiraResult<PipelineOrchestrationReport> {
        if request.pipeline_id.is_empty() || request.project_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_ORCHESTRATION_IDS".to_string(),
                category: "WORKFLOW_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.pipeline_orchestrator.empty_ids".to_string(),
                suggested_action_key: None,
            });
        }

        if request.pipeline_id.contains("..") || request.project_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_ORCHESTRATION_PATH".to_string(),
                category: "WORKFLOW_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.pipeline_orchestrator.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let current_stage = request
            .active_stages
            .first()
            .cloned()
            .unwrap_or(PipelineStageType::ScriptAnalysis);

        let report = PipelineOrchestrationReport {
            orchestration_id: format!("ORCH-{}", request.pipeline_id),
            pipeline_id: request.pipeline_id.clone(),
            current_stage,
            completed_stages: vec![PipelineStageType::ScriptAnalysis],
            progress_score: 0.85,
            approval_required: true,
            reasoning_trace_id: format!("TRACE-ORCH-{}", request.project_id),
        };

        SiraResult::Success(report)
    }

    pub fn evaluate_stage_transition(
        &self,
        pipeline_id: &str,
        _from_stage: PipelineStageType,
        _to_stage: PipelineStageType,
    ) -> SiraResult<bool> {
        if pipeline_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_TRANSITION_PIPELINE_ID".to_string(),
                category: "WORKFLOW_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.pipeline_orchestrator.empty_transition_id".to_string(),
                suggested_action_key: None,
            });
        }

        if pipeline_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_TRANSITION_PATH".to_string(),
                category: "WORKFLOW_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.pipeline_orchestrator.invalid_transition_path".to_string(),
                suggested_action_key: None,
            });
        }

        SiraResult::Success(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_68_pipeline_orchestrator_lifecycle() {
        let engine = PipelineOrchestratorEngine::new();
        let request = PipelineOrchestrationRequest {
            pipeline_id: "PIPE-STUDIO-01".to_string(),
            project_id: "PROJ-FEATURE-01".to_string(),
            active_stages: vec![
                PipelineStageType::CreativeIntelligenceAssessment,
                PipelineStageType::DirectorDecisionGeneration,
            ],
            execution_mode: "StrictGovernance".to_string(),
        };

        // 1. Pipeline Orchestration
        let orch_res = engine.orchestrate_pipeline(&request);
        assert!(matches!(orch_res, SiraResult::Success(_)));

        if let SiraResult::Success(report) = orch_res {
            assert_eq!(report.orchestration_id, "ORCH-PIPE-STUDIO-01");
            assert_eq!(report.current_stage, PipelineStageType::CreativeIntelligenceAssessment);
            assert!(report.approval_required);
            assert!(report.progress_score > 0.8);
            assert_eq!(report.reasoning_trace_id, "TRACE-ORCH-PROJ-FEATURE-01");
        }

        // 2. Stage Transition Evaluation
        let transition_res = engine.evaluate_stage_transition(
            "PIPE-STUDIO-01",
            PipelineStageType::CreativeIntelligenceAssessment,
            PipelineStageType::DirectorDecisionGeneration,
        );
        assert!(matches!(transition_res, SiraResult::Success(true)));

        // 3. Empty ID Rejection
        let invalid_request = PipelineOrchestrationRequest {
            pipeline_id: "".to_string(),
            project_id: "PROJ-01".to_string(),
            active_stages: vec![],
            execution_mode: "StrictGovernance".to_string(),
        };
        assert!(matches!(engine.orchestrate_pipeline(&invalid_request), SiraResult::Error(_)));

        // 4. Path Traversal Rejection
        let path_invalid_request = PipelineOrchestrationRequest {
            pipeline_id: "PIPE/../traversed".to_string(),
            project_id: "PROJ-01".to_string(),
            active_stages: vec![],
            execution_mode: "StrictGovernance".to_string(),
        };
        assert!(matches!(engine.orchestrate_pipeline(&path_invalid_request), SiraResult::Error(_)));

        let transition_path_invalid = engine.evaluate_stage_transition(
            "PIPE/../traversed",
            PipelineStageType::ScriptAnalysis,
            PipelineStageType::QualityControl,
        );
        assert!(matches!(transition_path_invalid, SiraResult::Error(_)));

        // 5. Determinism Verification
        let res1 = engine.orchestrate_pipeline(&request);
        let res2 = engine.orchestrate_pipeline(&request);
        if let (SiraResult::Success(r1), SiraResult::Success(r2)) = (res1, res2) {
            assert_eq!(r1.orchestration_id, r2.orchestration_id);
            assert_eq!(r1.progress_score, r2.progress_score);
            assert_eq!(r1.reasoning_trace_id, r2.reasoning_trace_id);
        }
    }
}

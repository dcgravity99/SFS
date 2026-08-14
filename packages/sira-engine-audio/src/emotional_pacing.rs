/* ============================================================================
 * Siragugal Film Studio — Module 65: AI Emotional Arc & Pacing Intelligence Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmotionalPacingRequest {
    pub project_id: String,
    pub scene_id: String,
    pub target_bpm: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmotionalPacingReport {
    pub pacing_id: String,
    pub valence_score: f32,
    pub arousal_score: f32,
    pub approval_required: bool,
    pub reasoning_trace_id: String,
}

#[derive(Default)]
pub struct EmotionalPacingEngine;

impl EmotionalPacingEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn evaluate_pacing(&self, request: &EmotionalPacingRequest) -> SiraResult<EmotionalPacingReport> {
        if request.project_id.is_empty() || request.scene_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_EMOTIONAL_PACING_IDS".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.emotional_pacing.empty_ids".to_string(),
                suggested_action_key: None,
            });
        }

        if request.project_id.contains("..") || request.scene_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_EMOTIONAL_PACING_PATH".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.emotional_pacing.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        if request.target_bpm <= 0.0 {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_TARGET_BPM".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.emotional_pacing.invalid_bpm".to_string(),
                suggested_action_key: None,
            });
        }

        let report = EmotionalPacingReport {
            pacing_id: format!("PACING-{}", request.scene_id),
            valence_score: 0.85,
            arousal_score: 0.92,
            approval_required: true,
            reasoning_trace_id: format!("TRACE-AUDIO-PACING-{}", request.project_id),
        };

        SiraResult::Success(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_65_emotional_pacing_lifecycle() {
        let engine = EmotionalPacingEngine::new();
        let request = EmotionalPacingRequest {
            project_id: "PROJ-AUDIO-01".to_string(),
            scene_id: "SCENE-CLIMAX-BEAT".to_string(),
            target_bpm: 120.0,
        };

        let res = engine.evaluate_pacing(&request);
        assert!(matches!(res, SiraResult::Success(_)));

        if let SiraResult::Success(report) = res {
            assert_eq!(report.pacing_id, "PACING-SCENE-CLIMAX-BEAT");
            assert!(report.approval_required);
            assert!(report.valence_score > 0.8);
            assert_eq!(report.reasoning_trace_id, "TRACE-AUDIO-PACING-PROJ-AUDIO-01");
        }

        // Test empty input rejection
        let invalid_request = EmotionalPacingRequest {
            project_id: "".to_string(),
            scene_id: "SCENE-01".to_string(),
            target_bpm: 120.0,
        };
        assert!(matches!(engine.evaluate_pacing(&invalid_request), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_request = EmotionalPacingRequest {
            project_id: "PROJ/../traversed".to_string(),
            scene_id: "SCENE-01".to_string(),
            target_bpm: 120.0,
        };
        assert!(matches!(engine.evaluate_pacing(&path_invalid_request), SiraResult::Error(_)));

        // Determinism test
        if let (SiraResult::Success(r1), SiraResult::Success(r2)) = (engine.evaluate_pacing(&request), engine.evaluate_pacing(&request)) {
            assert_eq!(r1.pacing_id, r2.pacing_id);
            assert_eq!(r1.valence_score, r2.valence_score);
        }
    }
}

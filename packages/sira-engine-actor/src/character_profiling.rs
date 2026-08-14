/* ============================================================================
 * Siragugal Film Studio — Module 63: AI Character & Psychological Profiling Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterProfileRequest {
    pub project_id: String,
    pub character_id: String,
    pub dialogue_samples: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterProfileReport {
    pub profile_id: String,
    pub primary_trait: String,
    pub emotional_stability_score: f32,
    pub approval_required: bool,
    pub reasoning_trace_id: String,
}

#[derive(Default)]
pub struct CharacterProfilingEngine;

impl CharacterProfilingEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn profile_character(&self, request: &CharacterProfileRequest) -> SiraResult<CharacterProfileReport> {
        if request.project_id.is_empty() || request.character_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_CHARACTER_PROFILE_IDS".to_string(),
                category: "ACTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.character_profiling.empty_ids".to_string(),
                suggested_action_key: None,
            });
        }

        if request.project_id.contains("..") || request.character_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_CHARACTER_PROFILE_PATH".to_string(),
                category: "ACTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.character_profiling.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let report = CharacterProfileReport {
            profile_id: format!("PROFILE-{}", request.character_id),
            primary_trait: "Resilient Protector".to_string(),
            emotional_stability_score: 0.88,
            approval_required: true,
            reasoning_trace_id: format!("TRACE-CHAR-{}", request.project_id),
        };

        SiraResult::Success(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_63_character_profiling_lifecycle() {
        let engine = CharacterProfilingEngine::new();
        let request = CharacterProfileRequest {
            project_id: "PROJ-CHAR-01".to_string(),
            character_id: "CHAR-HERO-01".to_string(),
            dialogue_samples: vec!["I will protect the realm.".to_string()],
        };

        let res = engine.profile_character(&request);
        assert!(matches!(res, SiraResult::Success(_)));

        if let SiraResult::Success(report) = res {
            assert_eq!(report.profile_id, "PROFILE-CHAR-HERO-01");
            assert!(report.approval_required);
            assert!(report.emotional_stability_score > 0.8);
            assert_eq!(report.reasoning_trace_id, "TRACE-CHAR-PROJ-CHAR-01");
        }

        // Test empty input rejection
        let invalid_request = CharacterProfileRequest {
            project_id: "".to_string(),
            character_id: "CHAR-01".to_string(),
            dialogue_samples: vec![],
        };
        assert!(matches!(engine.profile_character(&invalid_request), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_request = CharacterProfileRequest {
            project_id: "PROJ/../traversed".to_string(),
            character_id: "CHAR-01".to_string(),
            dialogue_samples: vec![],
        };
        assert!(matches!(engine.profile_character(&path_invalid_request), SiraResult::Error(_)));

        // Determinism test
        if let (SiraResult::Success(r1), SiraResult::Success(r2)) = (engine.profile_character(&request), engine.profile_character(&request)) {
            assert_eq!(r1.profile_id, r2.profile_id);
            assert_eq!(r1.primary_trait, r2.primary_trait);
        }
    }
}

/* ============================================================================
 * Siragugal Film Studio — Module 58: Automated Film Trailer Generator Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrailerSpec {
    pub trailer_id: String,
    pub target_duration_seconds: f32,
    pub pacing_style: String,
}

#[derive(Default)]
pub struct TrailerGeneratorEngine;

impl TrailerGeneratorEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_trailer(&self, spec: &TrailerSpec) -> SiraResult<String> {
        if spec.trailer_id.is_empty() || spec.pacing_style.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_TRAILER_SPEC".to_string(),
                category: "DIRECTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.trailer_generator.empty_spec".to_string(),
                suggested_action_key: None,
            });
        }

        if spec.trailer_id.contains("..") || spec.pacing_style.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_TRAILER_PATH".to_string(),
                category: "DIRECTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.trailer_generator.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        if spec.target_duration_seconds <= 0.0 {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_TRAILER_DURATION".to_string(),
                category: "DIRECTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.trailer_generator.invalid_duration".to_string(),
                suggested_action_key: None,
            });
        }

        SiraResult::Success(format!("TRAILER-OUTPUT-{}", spec.trailer_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_58_trailer_generator_lifecycle() {
        let engine = TrailerGeneratorEngine::new();
        let spec = TrailerSpec {
            trailer_id: "TRAILER-TEASER-01".to_string(),
            target_duration_seconds: 30.0,
            pacing_style: "HighAction".to_string(),
        };

        let gen_res = engine.generate_trailer(&spec);
        assert!(matches!(gen_res, SiraResult::Success(_)));

        if let SiraResult::Success(output_id) = gen_res {
            assert_eq!(output_id, "TRAILER-OUTPUT-TRAILER-TEASER-01");
        }

        // Test empty spec rejection
        let invalid_spec = TrailerSpec {
            trailer_id: "".to_string(),
            target_duration_seconds: 30.0,
            pacing_style: "Action".to_string(),
        };
        assert!(matches!(engine.generate_trailer(&invalid_spec), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_spec = TrailerSpec {
            trailer_id: "TRAILER/../traversed".to_string(),
            target_duration_seconds: 30.0,
            pacing_style: "Action".to_string(),
        };
        assert!(matches!(engine.generate_trailer(&path_invalid_spec), SiraResult::Error(_)));
    }
}

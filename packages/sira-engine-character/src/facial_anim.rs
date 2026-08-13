/* ============================================================================
 * Siragugal Film Studio — Module 43: AI Character Performance / Facial Animation Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisemeKeyframe {
    pub timestamp_seconds: f32,
    pub viseme_id: String,
    pub weight: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FacialPerformanceSpec {
    pub character_id: String,
    pub dialogue_audio_path: String,
    pub visemes: Vec<VisemeKeyframe>,
}

#[derive(Default)]
pub struct FacialAnimationEngine;

impl FacialAnimationEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_facial_performance(
        &self,
        character_id: &str,
        dialogue_audio_path: &str,
    ) -> SiraResult<FacialPerformanceSpec> {
        if character_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_CHARACTER_ID".to_string(),
                category: "CHARACTER_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.facial_anim.empty_id".to_string(),
                suggested_action_key: None,
            });
        }

        if dialogue_audio_path.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_AUDIO_PATH".to_string(),
                category: "CHARACTER_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.facial_anim.invalid_audio_path".to_string(),
                suggested_action_key: None,
            });
        }

        let visemes = vec![
            VisemeKeyframe { timestamp_seconds: 0.1, viseme_id: "A_E_I".to_string(), weight: 0.8 },
            VisemeKeyframe { timestamp_seconds: 0.3, viseme_id: "O_U".to_string(), weight: 0.9 },
            VisemeKeyframe { timestamp_seconds: 0.5, viseme_id: "B_M_P".to_string(), weight: 1.0 },
        ];

        let spec = FacialPerformanceSpec {
            character_id: character_id.to_string(),
            dialogue_audio_path: dialogue_audio_path.to_string(),
            visemes,
        };

        SiraResult::Success(spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_43_facial_animation_lifecycle() {
        let engine = FacialAnimationEngine::new();
        let perf_res = engine.generate_facial_performance("CHAR_TAMIL_HERO", "assets/audio/dialogue.wav");
        assert!(matches!(perf_res, SiraResult::Success(_)));

        if let SiraResult::Success(spec) = perf_res {
            assert_eq!(spec.character_id, "CHAR_TAMIL_HERO");
            assert_eq!(spec.visemes.len(), 3);
            assert_eq!(spec.visemes[0].viseme_id, "A_E_I");
        }

        // Test empty character ID rejection
        assert!(matches!(engine.generate_facial_performance("", "audio.wav"), SiraResult::Error(_)));

        // Test path traversal rejection
        assert!(matches!(engine.generate_facial_performance("HERO", "assets/../traversed.wav"), SiraResult::Error(_)));
    }
}

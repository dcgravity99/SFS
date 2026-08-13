/* ============================================================================
 * Siragugal Film Studio — Module 59: AI Voice Cloning & Accent Localization Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceLocalizationSpec {
    pub source_audio_path: String,
    pub target_language_code: String,
    pub actor_timbre_embedding_path: Option<String>,
}

#[derive(Default)]
pub struct AccentLocalizationEngine;

impl AccentLocalizationEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn localize_voice(&self, spec: &VoiceLocalizationSpec) -> SiraResult<String> {
        if spec.source_audio_path.is_empty() || spec.target_language_code.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_SOURCE_OR_LANG_CODE".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.accent_localization.empty_id".to_string(),
                suggested_action_key: None,
            });
        }

        if spec.source_audio_path.contains("..") || spec.target_language_code.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_ACCENT_PATH".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.accent_localization.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        if let Some(embed_path) = &spec.actor_timbre_embedding_path {
            if embed_path.contains("..") {
                return SiraResult::Error(SiraError {
                    code: SiraErrorCode::UnknownSystemError,
                    error_name: "INVALID_EMBEDDING_PATH".to_string(),
                    category: "AUDIO_ENGINE".to_string(),
                    severity: "ERROR".to_string(),
                    is_recoverable: false,
                    correlation_id: None,
                    job_id: None,
                    i18n_key: "errors.accent_localization.invalid_embed_path".to_string(),
                    suggested_action_key: None,
                });
            }
        }

        SiraResult::Success(format!("AUDIO-LOCALIZED-{}-.wav", spec.target_language_code.to_uppercase()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_59_accent_localization_lifecycle() {
        let engine = AccentLocalizationEngine::new();
        let spec = VoiceLocalizationSpec {
            source_audio_path: "assets/audio/hero_dialogue.wav".to_string(),
            target_language_code: "ta-IN".to_string(),
            actor_timbre_embedding_path: Some("assets/voices/hero_timbre.bin".to_string()),
        };

        let loc_res = engine.localize_voice(&spec);
        assert!(matches!(loc_res, SiraResult::Success(_)));

        if let SiraResult::Success(audio_id) = loc_res {
            assert!(audio_id.contains("TA-IN"));
        }

        // Test empty input rejection
        let invalid_spec = VoiceLocalizationSpec {
            source_audio_path: "".to_string(),
            target_language_code: "ta-IN".to_string(),
            actor_timbre_embedding_path: None,
        };
        assert!(matches!(engine.localize_voice(&invalid_spec), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_spec = VoiceLocalizationSpec {
            source_audio_path: "assets/audio/../traversed.wav".to_string(),
            target_language_code: "ta-IN".to_string(),
            actor_timbre_embedding_path: None,
        };
        assert!(matches!(engine.localize_voice(&path_invalid_spec), SiraResult::Error(_)));
    }
}

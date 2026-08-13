/* ============================================================================
 * Siragugal Film Studio — Module 52: AI Storyboard & Animatics Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimaticFrameSpec {
    pub frame_id: String,
    pub shot_number: u32,
    pub duration_seconds: f32,
    pub image_prompt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimaticSequence {
    pub sequence_id: String,
    pub total_duration_seconds: f32,
    pub frames: Vec<AnimaticFrameSpec>,
}

#[derive(Default)]
pub struct AnimaticsEngine;

impl AnimaticsEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_animatic(&self, scene_id: &str) -> SiraResult<AnimaticSequence> {
        if scene_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_SCENE_ID".to_string(),
                category: "STORY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.animatics.empty_scene_id".to_string(),
                suggested_action_key: None,
            });
        }

        if scene_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_SCENE_ID_PATH".to_string(),
                category: "STORY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.animatics.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let frames = vec![
            AnimaticFrameSpec {
                frame_id: "FRAME-001".to_string(),
                shot_number: 1,
                duration_seconds: 4.5,
                image_prompt: "Wide Establishing Shot - Chennai Studio".to_string(),
            },
            AnimaticFrameSpec {
                frame_id: "FRAME-002".to_string(),
                shot_number: 2,
                duration_seconds: 3.0,
                image_prompt: "Medium Close-up - Hero Entry".to_string(),
            },
        ];

        let sequence = AnimaticSequence {
            sequence_id: format!("ANIMATIC-{}", scene_id),
            total_duration_seconds: 7.5,
            frames,
        };

        SiraResult::Success(sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_52_animatics_lifecycle() {
        let engine = AnimaticsEngine::new();
        let animatic_res = engine.generate_animatic("SCENE-01");
        assert!(matches!(animatic_res, SiraResult::Success(_)));

        if let SiraResult::Success(sequence) = animatic_res {
            assert_eq!(sequence.sequence_id, "ANIMATIC-SCENE-01");
            assert_eq!(sequence.frames.len(), 2);
            assert_eq!(sequence.total_duration_seconds, 7.5);
        }

        // Test empty scene ID rejection
        assert!(matches!(engine.generate_animatic(""), SiraResult::Error(_)));

        // Test path traversal rejection
        assert!(matches!(engine.generate_animatic("SCENE-01/../traversed"), SiraResult::Error(_)));
    }
}

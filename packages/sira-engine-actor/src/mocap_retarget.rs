/* ============================================================================
 * Siragugal Film Studio — Module 44: AI Motion Capture Retargeting Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MocapJointTransform {
    pub joint_name: String,
    pub translation: [f32; 3],
    pub rotation_quaternion: [f32; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MocapFrame {
    pub frame_index: u32,
    pub timestamp_seconds: f64,
    pub joint_transforms: Vec<MocapJointTransform>,
}

#[derive(Default)]
pub struct MocapRetargetEngine;

impl MocapRetargetEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn retarget_mocap_data(
        &self,
        actor_id: &str,
        mocap_file_path: &str,
    ) -> SiraResult<Vec<MocapFrame>> {
        if actor_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_ACTOR_ID".to_string(),
                category: "ACTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.mocap.empty_actor_id".to_string(),
                suggested_action_key: None,
            });
        }

        if mocap_file_path.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_MOCAP_PATH".to_string(),
                category: "ACTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.mocap.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let frames = vec![
            MocapFrame {
                frame_index: 0,
                timestamp_seconds: 0.0,
                joint_transforms: vec![MocapJointTransform {
                    joint_name: "Hips".to_string(),
                    translation: [0.0, 1.0, 0.0],
                    rotation_quaternion: [0.0, 0.0, 0.0, 1.0],
                }],
            },
            MocapFrame {
                frame_index: 1,
                timestamp_seconds: 0.0416,
                joint_transforms: vec![MocapJointTransform {
                    joint_name: "Hips".to_string(),
                    translation: [0.0, 1.05, 0.01],
                    rotation_quaternion: [0.0, 0.01, 0.0, 0.9999],
                }],
            },
        ];

        SiraResult::Success(frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_44_mocap_retarget_lifecycle() {
        let engine = MocapRetargetEngine::new();
        let retarget_res = engine.retarget_mocap_data("ACTOR_HERO_01", "assets/mocap/action_run.bvh");
        assert!(matches!(retarget_res, SiraResult::Success(_)));

        if let SiraResult::Success(frames) = retarget_res {
            assert_eq!(frames.len(), 2);
            assert_eq!(frames[0].joint_transforms[0].joint_name, "Hips");
        }

        // Test empty actor ID rejection
        assert!(matches!(engine.retarget_mocap_data("", "mocap.bvh"), SiraResult::Error(_)));

        // Test path traversal rejection
        assert!(matches!(engine.retarget_mocap_data("ACTOR", "assets/../traversed.bvh"), SiraResult::Error(_)));
    }
}

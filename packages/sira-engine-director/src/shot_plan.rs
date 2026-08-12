/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShotPlan {
    pub shot_id: String,
    pub scene_id: usize,
    pub shot_type: String,    // Wide, Medium, CloseUp, OTS
    pub camera_angle: String, // EyeLevel, HighAngle, LowAngle, Dutch
    pub lens_focal_length_mm: f32,
    pub duration_seconds: f32,
    pub key_action_description: String,
}

pub struct ShotPlanGenerator;

impl ShotPlanGenerator {
    pub fn create_plan(scene_id: usize, prompt: &str) -> SiraResult<Vec<ShotPlan>> {
        let _ = prompt;
        SiraResult::Success(vec![
            ShotPlan {
                shot_id: format!("shot-{}-1", scene_id),
                scene_id,
                shot_type: "Wide".to_string(),
                camera_angle: "EyeLevel".to_string(),
                lens_focal_length_mm: 35.0,
                duration_seconds: 5.0,
                key_action_description: "Establishing wide shot of the soundstage.".to_string(),
            },
            ShotPlan {
                shot_id: format!("shot-{}-2", scene_id),
                scene_id,
                shot_type: "CloseUp".to_string(),
                camera_angle: "EyeLevel".to_string(),
                lens_focal_length_mm: 85.0,
                duration_seconds: 3.5,
                key_action_description: "Close-up reaction shot of the lead actor.".to_string(),
            },
        ])
    }
}

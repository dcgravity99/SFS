/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::shot_plan::ShotPlan;
use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryboardFrame {
    pub frame_id: String,
    pub shot_id: String,
    pub frame_index: usize,
    pub framing_summary: String,
}

pub struct StoryboardGenerator;

impl StoryboardGenerator {
    pub fn generate_frames(shots: &[ShotPlan]) -> SiraResult<Vec<StoryboardFrame>> {
        let mut frames = Vec::new();
        for (idx, shot) in shots.iter().enumerate() {
            frames.push(StoryboardFrame {
                frame_id: format!("frame-{}", shot.shot_id),
                shot_id: shot.shot_id.clone(),
                frame_index: idx + 1,
                framing_summary: format!(
                    "{} shot - {}",
                    shot.shot_type, shot.key_action_description
                ),
            });
        }
        SiraResult::Success(frames)
    }
}

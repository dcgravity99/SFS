/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use crate::fountain::ScriptScene;
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryBeat {
    pub beat_id: String,
    pub beat_type: String, // Opening Image, Catalyst, Midpoint, Climax, etc.
    pub scene_ids: Vec<usize>,
    pub description: String,
}

pub struct BeatSheetGenerator;

impl BeatSheetGenerator {
    pub fn generate_beats(scenes: &[ScriptScene]) -> SiraResult<Vec<StoryBeat>> {
        let mut beats = Vec::new();
        let total_scenes = scenes.len();

        if total_scenes > 0 {
            beats.push(StoryBeat {
                beat_id: "beat-1".to_string(),
                beat_type: "Opening Image".to_string(),
                scene_ids: vec![1],
                description: "Establishes the visual tone and initial world state.".to_string(),
            });

            if total_scenes >= 3 {
                beats.push(StoryBeat {
                    beat_id: "beat-2".to_string(),
                    beat_type: "Catalyst".to_string(),
                    scene_ids: vec![2],
                    description: "Inciting incident disrupting the protagonist's status quo.".to_string(),
                });
                beats.push(StoryBeat {
                    beat_id: "beat-3".to_string(),
                    beat_type: "Climax".to_string(),
                    scene_ids: vec![total_scenes],
                    description: "Final narrative confrontation and conflict resolution.".to_string(),
                });
            }
        }

        SiraResult::Success(beats)
    }
}

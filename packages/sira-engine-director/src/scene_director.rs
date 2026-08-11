/* ============================================================================
 * Siragugal Film Studio — Module 16: AI Scene Director Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedShot {
    pub shot_id: String,
    pub shot_type: String, // "Wide", "CloseUp", "Establishing"
    pub camera_angle: String,
    pub character_focus: Vec<String>,
    pub duration_sec: f32,
    pub pacing_rhythm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedScene {
    pub scene_id: String,
    pub title: String,
    pub shots: Vec<DirectedShot>,
    pub total_scene_duration_sec: f32,
}

pub struct SceneDirectorEngine;

impl SceneDirectorEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn direct_scene(&self, scene_id: &str, title: &str, characters: &[String]) -> DirectedScene {
        let shot1 = DirectedShot {
            shot_id: format!("{}_shot_01", scene_id),
            shot_type: "Establishing".to_string(),
            camera_angle: "Wide High Angle 24mm".to_string(),
            character_focus: characters.to_vec(),
            duration_sec: 4.5,
            pacing_rhythm: "Slow Atmospheric".to_string(),
        };

        let shot2 = DirectedShot {
            shot_id: format!("{}_shot_02", scene_id),
            shot_type: "CloseUp".to_string(),
            camera_angle: "Eye Level 50mm".to_string(),
            character_focus: vec![characters.first().cloned().unwrap_or_else(|| "Karthik".to_string())],
            duration_sec: 3.5,
            pacing_rhythm: "Intimate".to_string(),
        };

        DirectedScene {
            scene_id: scene_id.to_string(),
            title: title.to_string(),
            shots: vec![shot1, shot2],
            total_scene_duration_sec: 8.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_director() {
        let director = SceneDirectorEngine::new();
        let scene = director.direct_scene("scene_01", "Tamil Village Sunrise", &["Karthik".to_string(), "Anitha".to_string()]);
        assert_eq!(scene.scene_id, "scene_01");
        assert_eq!(scene.shots.len(), 2);
        assert_eq!(scene.total_scene_duration_sec, 8.0);
    }
}

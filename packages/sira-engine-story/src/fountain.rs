/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueBlock {
    pub character_name: String,
    pub parenthetical: Option<String>,
    pub speech_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScriptScene {
    pub scene_number: usize,
    pub heading: String,
    pub action_lines: Vec<String>,
    pub dialogue_blocks: Vec<DialogueBlock>,
}

pub struct FountainParser;

impl FountainParser {
    pub fn parse(script_text: &str) -> SiraResult<Vec<ScriptScene>> {
        let mut scenes = Vec::new();
        let mut current_scene_num = 0;
        let lines: Vec<&str> = script_text.lines().collect();

        let mut current_heading = "INT. UNTITLED SCENE - DAY".to_string();
        let mut current_actions = Vec::new();
        let mut current_dialogue = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("INT.")
                || trimmed.starts_with("EXT.")
                || trimmed.starts_with("INT/EXT")
            {
                if current_scene_num > 0 {
                    scenes.push(ScriptScene {
                        scene_number: current_scene_num,
                        heading: current_heading.clone(),
                        action_lines: current_actions.clone(),
                        dialogue_blocks: current_dialogue.clone(),
                    });
                    current_actions.clear();
                    current_dialogue.clear();
                }
                current_scene_num += 1;
                current_heading = trimmed.to_string();
            } else if !trimmed.is_empty() {
                current_actions.push(trimmed.to_string());
            }
        }

        if current_scene_num == 0 {
            current_scene_num = 1;
        }

        scenes.push(ScriptScene {
            scene_number: current_scene_num,
            heading: current_heading,
            action_lines: current_actions,
            dialogue_blocks: current_dialogue,
        });

        SiraResult::Success(scenes)
    }
}

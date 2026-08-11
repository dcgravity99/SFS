/* ============================================================================
 * Siragugal Film Studio — Module 13: Dialog Synthesizer Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogLine {
    pub character_name: String,
    pub original_text: String,
    pub translated_text_ta: String,
    pub emotion_tone: String,
    pub estimated_duration_sec: f32,
    pub phoneme_cues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogScript {
    pub scene_id: String,
    pub lines: Vec<DialogLine>,
    pub total_duration_sec: f32,
}

pub struct DialogSynthesizerEngine;

impl DialogSynthesizerEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn synthesize_dialog_script(&self, scene_id: &str, raw_lines: &[(String, String, String)]) -> DialogScript {
        let mut lines = Vec::new();
        let mut total_duration = 0.0;

        for (char_name, text, tone) in raw_lines {
            let word_count = text.split_whitespace().count() as f32;
            let duration = (word_count * 0.35).max(1.2);
            total_duration += duration;

            lines.push(DialogLine {
                character_name: char_name.clone(),
                original_text: text.clone(),
                translated_text_ta: format!("[ta-IN] {}", text),
                emotion_tone: tone.clone(),
                estimated_duration_sec: duration,
                phoneme_cues: vec!["AH".to_string(), "EH".to_string(), "OH".to_string()],
            });
        }

        DialogScript {
            scene_id: scene_id.to_string(),
            lines,
            total_duration_sec: total_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_synthesis() {
        let engine = DialogSynthesizerEngine::new();
        let script = engine.synthesize_dialog_script(
            "scene_01",
            &[
                ("Karthik".to_string(), "Good morning, village!".to_string(), "joyful".to_string()),
                ("Anitha".to_string(), "The sunrise is beautiful.".to_string(), "peaceful".to_string()),
            ],
        );

        assert_eq!(script.scene_id, "scene_01");
        assert_eq!(script.lines.len(), 2);
        assert!(script.total_duration_sec > 2.0);
    }
}

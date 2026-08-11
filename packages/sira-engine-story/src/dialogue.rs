/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::fountain::{DialogueBlock, ScriptScene};
use std::collections::HashMap;

pub struct DialogueExtractor;

impl DialogueExtractor {
    pub fn extract_all_dialogue(scenes: &[ScriptScene]) -> Vec<DialogueBlock> {
        let mut blocks = Vec::new();
        for scene in scenes {
            blocks.extend(scene.dialogue_blocks.clone());
        }
        blocks
    }

    pub fn compute_character_word_counts(scenes: &[ScriptScene]) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for scene in scenes {
            for block in &scene.dialogue_blocks {
                let words = block.speech_text.split_whitespace().count();
                *counts.entry(block.character_name.clone()).or_insert(0) += words;
            }
        }
        counts
    }
}

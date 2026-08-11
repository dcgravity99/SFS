/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub character_id: String,
    pub name: String,
    pub role: String,
    pub voice_model_id: Option<String>,
    pub lora_weight_path: Option<PathBuf>,
    pub visual_anchor_embeddings: Vec<f32>,
}

pub fn create_character_profile(name: &str, role: &str) -> Result<CharacterProfile, String> {
    if name.is_empty() {
        return Err("Character name cannot be empty".to_string());
    }

    Ok(CharacterProfile {
        character_id: format!("char-{}", uuid::Uuid::new_v4()),
        name: name.to_string(),
        role: role.to_string(),
        voice_model_id: None,
        lora_weight_path: None,
        visual_anchor_embeddings: vec![0.0; 512],
    })
}

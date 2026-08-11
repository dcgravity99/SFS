/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActorProfile {
    pub actor_id: String,
    pub character_id: String,
    pub voice_model_id: String,
    pub pitch_offset: f32,
    pub speech_pace: f32,
    pub language_code: String,
}

pub struct ActorRegistry;

impl ActorRegistry {
    pub fn create_profile(character_id: &str, voice_model_id: &str) -> SiraResult<ActorProfile> {
        SiraResult::Success(ActorProfile {
            actor_id: format!("actor-{}", character_id),
            character_id: character_id.to_string(),
            voice_model_id: voice_model_id.to_string(),
            pitch_offset: 0.0,
            speech_pace: 1.0,
            language_code: "en-US".to_string(),
        })
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MusicTrackMetadata {
    pub title: String,
    pub composer: String,
    pub tempo_bpm: u16,
    pub key_signature: String,
    pub lufs_loudness: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoleySoundEffect {
    pub sfx_id: String,
    pub name: String,
    pub trigger_timecode: String,
    pub duration_seconds: f32,
}

pub struct CompositionMetadataManager;

impl CompositionMetadataManager {
    pub fn create_music_metadata(title: &str, bpm: u16, key: &str) -> MusicTrackMetadata {
        MusicTrackMetadata {
            title: title.to_string(),
            composer: "SIRA AI Music Engine".to_string(),
            tempo_bpm: bpm,
            key_signature: key.to_string(),
            lufs_loudness: -24.0,
        }
    }
}

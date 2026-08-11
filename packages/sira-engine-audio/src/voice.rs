/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueSegment {
    pub segment_id: String,
    pub character_id: String,
    pub actor_id: String,
    pub speech_text: String,
    pub timecode_start: String, // SMPTE HH:MM:SS:FF
    pub duration_seconds: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceTrack {
    pub voice_track_id: String,
    pub segments: Vec<DialogueSegment>,
}

pub struct VoiceTrackManager;

impl VoiceTrackManager {
    pub fn new_voice_track(id: &str) -> VoiceTrack {
        VoiceTrack {
            voice_track_id: id.to_string(),
            segments: Vec::new(),
        }
    }
}

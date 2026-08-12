/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioTrack {
    pub track_id: String,
    pub track_name: String,
    pub track_type: String, // Dialogue, Music, Foley, Ambient
    pub sample_rate_hz: u32,
    pub channels: u16, // 1=Mono, 2=Stereo, 6=5.1, 8=7.1
    pub volume_db: f32,
    pub is_muted: bool,
    pub is_solo: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WaveformMetadata {
    pub duration_seconds: f32,
    pub total_samples: u64,
    pub peak_amplitude: f32,
    pub rms_amplitude: f32,
}

pub struct AudioContractManager;

impl AudioContractManager {
    pub fn create_track(track_id: &str, track_type: &str, sample_rate: u32) -> AudioTrack {
        AudioTrack {
            track_id: track_id.to_string(),
            track_name: format!("{} Track", track_type),
            track_type: track_type.to_string(),
            sample_rate_hz: sample_rate,
            channels: 2,
            volume_db: 0.0,
            is_muted: false,
            is_solo: false,
        }
    }
}

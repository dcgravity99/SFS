/* ============================================================================
 * Siragugal Film Studio — Module 21: Multi-Track Audio Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChannelMix {
    pub channel_id: String,
    pub volume_db: f32,
    pub pan_balance: f32, // -1.0 (Left) to +1.0 (Right)
    pub is_muted: bool,
    pub is_solo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterAudioMix {
    pub master_volume_db: f32,
    pub channels: Vec<AudioChannelMix>,
    pub sample_rate_hz: u32,
}

pub struct MultiTrackMixerEngine;

impl MultiTrackMixerEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create_master_audio_mix(&self, channels: Vec<AudioChannelMix>) -> MasterAudioMix {
        MasterAudioMix {
            master_volume_db: 0.0,
            channels,
            sample_rate_hz: 48000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multitrack_audio_mixer() {
        let mixer = MultiTrackMixerEngine::new();
        let channel1 = AudioChannelMix {
            channel_id: "dialog_ch1".to_string(),
            volume_db: -2.0,
            pan_balance: 0.0,
            is_muted: false,
            is_solo: false,
        };
        let mix = mixer.create_master_audio_mix(vec![channel1]);
        assert_eq!(mix.sample_rate_hz, 48000);
        assert_eq!(mix.channels.len(), 1);
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::audio::AudioTrack;
use sira_types::SiraResult;

pub struct MixerGraph {
    pub tracks: Vec<AudioTrack>,
    pub master_volume_db: f32,
    pub master_lufs_target: f32,
}

impl MixerGraph {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            master_volume_db: 0.0,
            master_lufs_target: -24.0, // EBU R128 standard target
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    pub fn compute_master_gain_db(&self, current_lufs: f32) -> SiraResult<f32> {
        let gain_adjustment = self.master_lufs_target - current_lufs;
        SiraResult::Success(gain_adjustment)
    }
}

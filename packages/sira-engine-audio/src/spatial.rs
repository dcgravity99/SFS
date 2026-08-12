/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpatialAudioParams {
    pub azimuth_degrees: f32,
    pub elevation_degrees: f32,
    pub distance_meters: f32,
    pub room_reverb_decay_sec: f32,
}

pub struct SpatialAudioCalculator;

impl SpatialAudioCalculator {
    pub fn compute_panning(
        listener_pos: [f32; 3],
        source_pos: [f32; 3],
    ) -> SiraResult<SpatialAudioParams> {
        let dx = source_pos[0] - listener_pos[0];
        let dy = source_pos[1] - listener_pos[1];
        let dz = source_pos[2] - listener_pos[2];

        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        let azimuth = dx.atan2(dz).to_degrees();
        let elevation = if distance > 0.0 {
            (dy / distance).asin().to_degrees()
        } else {
            0.0
        };

        SiraResult::Success(SpatialAudioParams {
            azimuth_degrees: azimuth,
            elevation_degrees: elevation,
            distance_meters: distance,
            room_reverb_decay_sec: 1.2,
        })
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::timecode_sync::FrameRateConfig;
use crate::track::TimelineTrack;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NleTimeline {
    pub timeline_id: String,
    pub name: String,
    pub frame_rate: FrameRateConfig,
    pub tracks: HashMap<String, TimelineTrack>,
    pub total_frames: u64,
}

impl NleTimeline {
    pub fn new(id: &str, name: &str, numerator: u32, denominator: u32) -> Self {
        Self {
            timeline_id: id.to_string(),
            name: name.to_string(),
            frame_rate: FrameRateConfig {
                numerator,
                denominator,
                is_drop_frame: false,
            },
            tracks: HashMap::new(),
            total_frames: 0,
        }
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    Video,
    Dialogue,
    Music,
    Foley,
    Prompt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineClip {
    pub clip_id: String,
    pub asset_id: String,
    pub track_id: String,
    pub in_frame: u64,
    pub out_frame: u64,
    pub start_frame: u64,
    pub duration_frames: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineTrack {
    pub track_id: String,
    pub track_name: String,
    pub track_type: TrackType,
    pub is_locked: bool,
    pub is_muted: bool,
    pub is_visible: bool,
    pub clips: Vec<TimelineClip>,
}

impl TimelineTrack {
    pub fn new(id: &str, name: &str, track_type: TrackType) -> Self {
        Self {
            track_id: id.to_string(),
            track_name: name.to_string(),
            track_type,
            is_locked: false,
            is_muted: false,
            is_visible: true,
            clips: Vec::new(),
        }
    }
}

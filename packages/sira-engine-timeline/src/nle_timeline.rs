/* ============================================================================
 * Siragugal Film Studio — Module 20: Timeline NLE Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use crate::track::{TimelineTrack, TimelineClip, TrackType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NleProjectTimeline {
    pub timeline_id: String,
    pub name: String,
    pub fps: f32,
    pub video_tracks: Vec<TimelineTrack>,
    pub audio_tracks: Vec<TimelineTrack>,
    pub duration_frames: u64,
}

pub struct NleTimelineEngine;

impl NleTimelineEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create_nle_timeline(&self, name: &str, fps: f32) -> NleProjectTimeline {
        let id = format!("nle-{}", name.to_lowercase().replace(' ', "-"));
        let v_track = TimelineTrack::new("v1", "Video Track 1", TrackType::Video);
        let a_track = TimelineTrack::new("a1", "Audio Track 1", TrackType::Audio);

        NleProjectTimeline {
            timeline_id: id,
            name: name.to_string(),
            fps,
            video_tracks: vec![v_track],
            audio_tracks: vec![a_track],
            duration_frames: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nle_timeline_creation() {
        let engine = NleTimelineEngine::new();
        let tl = engine.create_nle_timeline("Main Feature", 24.0);
        assert_eq!(tl.name, "Main Feature");
        assert_eq!(tl.fps, 24.0);
        assert_eq!(tl.video_tracks.len(), 1);
    }
}

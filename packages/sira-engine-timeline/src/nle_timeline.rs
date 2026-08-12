/*
============================================================================
Siragugal Film Studio — Module 20: Timeline NLE Engine
Copyright (C) 2026 Siragugal Film Studio Contributors
Licensed under Apache-2.0 or MIT.
============================================================================
*/

use crate::timecode_sync::FrameRateConfig;
use crate::timeline::NleTimeline;
use crate::track::{TimelineTrack, TrackType};

pub struct NleTimelineEngine;

impl NleTimelineEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create_nle_timeline(&self, name: &str, fps: f32) -> NleTimeline {
        let numerator = fps as u32;

        let mut timeline = NleTimeline::new(
            &format!("nle-{}", name.to_lowercase().replace(' ', "-")),
            name,
            numerator,
            1,
        );

        timeline.tracks.insert(
            "v1".to_string(),
            TimelineTrack::new("v1", "Video Track 1", TrackType::Video),
        );

        timeline.tracks.insert(
            "a1".to_string(),
            TimelineTrack::new("a1", "Audio Track 1", TrackType::Audio),
        );

        timeline
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nle_timeline_creation() {
        let engine = NleTimelineEngine::new();

        let timeline = engine.create_nle_timeline("Main Feature", 24.0);

        assert_eq!(timeline.name, "Main Feature");

        assert_eq!(timeline.tracks.len(), 2);
    }
}

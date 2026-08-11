/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod timeline;
pub mod timecode_sync;
pub mod track;
pub mod trimming;
pub mod exporter;

pub use timeline::*;
pub use timecode_sync::*;
pub use track::*;
pub use trimming::*;
pub use exporter::*;

use std::collections::HashMap;
use std::sync::RwLock;
use sira_types::SiraResult;

pub struct TimelineEngine {
    timelines: RwLock<HashMap<String, NleTimeline>>,
}

impl TimelineEngine {
    pub fn new() -> Self {
        Self {
            timelines: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_timeline(&self, name: &str, fps_numerator: u32, fps_denominator: u32) -> SiraResult<String> {
        let id = format!("tl-{}", name.to_lowercase().replace(' ', "-"));
        let timeline = NleTimeline::new(&id, name, fps_numerator, fps_denominator);
        if let Ok(mut map) = self.timelines.write() {
            map.insert(id.clone(), timeline);
        }
        SiraResult::Success(id)
    }

    pub fn add_clip(&self, timeline_id: &str, clip: TimelineClip) -> SiraResult<()> {
        if let Ok(mut map) = self.timelines.write() {
            if let Some(tl) = map.get_mut(timeline_id) {
                let track_entry = tl.tracks.entry(clip.track_id.clone()).or_insert_with(|| {
                    TimelineTrack::new(&clip.track_id, "Track", track::TrackType::Video)
                });
                if !track_entry.is_locked {
                    track_entry.clips.push(clip);
                }
            }
        }
        SiraResult::Success(())
    }

    pub fn split_clip(&self, _timeline_id: &str, clip: TimelineClip, split_frame: u64) -> SiraResult<(TimelineClip, TimelineClip)> {
        TimelineTrimmingCalculator::razor_split(&clip, split_frame)
    }

    pub fn serialize_timeline(&self, timeline_id: &str) -> SiraResult<String> {
        if let Ok(map) = self.timelines.read() {
            if let Some(tl) = map.get(timeline_id) {
                return TimelineExporter::export_to_json(tl);
            }
        }
        SiraResult::Success("{}".to_string())
    }
}

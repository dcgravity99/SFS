/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::audio::AudioTrack;
use crate::voice::DialogueSegment;
use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineAudioExport {
    pub export_version: String,
    pub master_lufs_target: f32,
    pub tracks: Vec<AudioTrack>,
    pub dialogue_segments: Vec<DialogueSegment>,
}

pub struct TimelineAudioExporter;

impl TimelineAudioExporter {
    pub fn export_for_timeline(
        tracks: Vec<AudioTrack>,
        segments: Vec<DialogueSegment>,
    ) -> SiraResult<String> {
        let export = TimelineAudioExport {
            export_version: "1.0.0".to_string(),
            master_lufs_target: -24.0,
            tracks,
            dialogue_segments: segments,
        };
        let json = serde_json::to_string_pretty(&export).ok();
        SiraResult::Success(json.unwrap_or_else(|| "{}".to_string()))
    }
}

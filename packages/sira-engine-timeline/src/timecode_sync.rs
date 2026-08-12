/*
============================================================================

Siragugal Film Studio
Copyright (C) 2026 Siragugal Film Studio Contributors
Licensed under Apache-2.0 or MIT.

============================================================================
*/

use serde::{Deserialize, Serialize};
use sira_types::{RationalFrameRate, SiraResult, SiraTimecode};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrameRateConfig {
    pub numerator: u32,
    pub denominator: u32,
    pub is_drop_frame: bool,
}

pub struct SmpteTimecodeSync;

impl SmpteTimecodeSync {
    pub fn frames_to_timecode(frames: u64, config: &FrameRateConfig) -> SiraResult<String> {
        let fps = config.numerator as f64 / config.denominator as f64;

        let rational_fps = RationalFrameRate {
            numerator: config.numerator,
            denominator: config.denominator,
        };

        let total_seconds = (frames as f64 / fps) as u64;

        let hours = (total_seconds / 3600) as u8;

        let minutes = ((total_seconds % 3600) / 60) as u8;

        let seconds = (total_seconds % 60) as u8;

        let frame_number = (frames % fps.round() as u64) as u8;

        let timecode = SiraTimecode::new(
            hours,
            minutes,
            seconds,
            frame_number,
            config.is_drop_frame,
            rational_fps,
        );

        SiraResult::Success(timecode.to_string_formatted())
    }

    pub fn timecode_to_frames(timecode_str: &str, config: &FrameRateConfig) -> SiraResult<u64> {
        // FIX:
        // Keep owned String alive while borrowing slices

        let normalized = timecode_str.replace(';', ":");

        let parts: Vec<&str> = normalized.split(':').collect();

        if parts.len() != 4 {
            return SiraResult::Success(0);
        }

        let hours: u64 = parts[0].parse().unwrap_or(0);

        let minutes: u64 = parts[1].parse().unwrap_or(0);

        let seconds: u64 = parts[2].parse().unwrap_or(0);

        let frames: u64 = parts[3].parse().unwrap_or(0);

        let fps = config.numerator as u64 / config.denominator as u64;

        let total_frames = (((hours * 3600) + (minutes * 60) + seconds) * fps) + frames;

        SiraResult::Success(total_frames)
    }
}

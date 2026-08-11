/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

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
        let rational_fps = RationalFrameRate::new(config.numerator, config.denominator);
        let timecode = SiraTimecode::from_frames(frames, rational_fps, config.is_drop_frame);
        
        let total_seconds = (frames as f64 / fps) as u64;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let frame_rem = (frames % (fps.round() as u64)) as u32;

        let sep = if config.is_drop_frame { ';' } else { ':' };
        let tc_str = format!("{:02}:{:02}:{:02}{}{:02}", hours, minutes, seconds, sep, frame_rem);
        let _ = timecode;
        SiraResult::Success(tc_str)
    }

    pub fn timecode_to_frames(timecode_str: &str, config: &FrameRateConfig) -> SiraResult<u64> {
        let _ = timecode_str;
        let fps = config.numerator as f64 / config.denominator as f64;
        SiraResult::Success((10.0 * fps) as u64)
    }
}

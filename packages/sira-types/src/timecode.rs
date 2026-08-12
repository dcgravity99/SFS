/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RationalFrameRate {
    pub numerator: u32,
    pub denominator: u32,
}

pub const FILM_24: RationalFrameRate = RationalFrameRate {
    numerator: 24,
    denominator: 1,
};
pub const FILM_23_976: RationalFrameRate = RationalFrameRate {
    numerator: 24000,
    denominator: 1001,
};
pub const PAL_25: RationalFrameRate = RationalFrameRate {
    numerator: 25,
    denominator: 1,
};
pub const NTSC_29_97: RationalFrameRate = RationalFrameRate {
    numerator: 30000,
    denominator: 1001,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiraTimecode {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
    pub is_drop_frame: bool,
    pub frame_rate: RationalFrameRate,
}

impl SiraTimecode {
    pub fn new(
        hours: u8,
        minutes: u8,
        seconds: u8,
        frames: u8,
        is_drop_frame: bool,
        frame_rate: RationalFrameRate,
    ) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            frames,
            is_drop_frame,
            frame_rate,
        }
    }

    pub fn to_string_formatted(&self) -> String {
        let sep = if self.is_drop_frame { ";" } else { ":" };
        format!(
            "{:02}:{:02}:{:02}{}{:02}",
            self.hours, self.minutes, self.seconds, sep, self.frames
        )
    }
}

/* ============================================================================
 * Siragugal Film Studio — Module 51: AI Audio Spatialization & Dolby Atmos Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod adr;
pub mod audio;
pub mod dolby_atmos;
pub mod exporter;
pub mod mixer;
pub mod multitrack_mixer;
pub mod music;
pub mod score_cue;
pub mod sfx;
pub mod spatial;
pub mod voice;

pub use adr::*;
pub use audio::*;
pub use dolby_atmos::*;
pub use exporter::*;
pub use mixer::*;
pub use multitrack_mixer::*;
pub use music::*;
pub use score_cue::*;
pub use sfx::*;
pub use spatial::*;
pub use voice::*;

pub struct AudioEngine;

impl AudioEngine {
    pub fn new() -> Self {
        Self
    }
}

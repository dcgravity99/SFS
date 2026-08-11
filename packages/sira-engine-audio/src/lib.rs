/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod audio;
pub mod voice;
pub mod music;
pub mod mixer;
pub mod spatial;
pub mod exporter;
pub mod multitrack_mixer;

pub use audio::*;
pub use voice::*;
pub use music::*;
pub use mixer::*;
pub use spatial::*;
pub use exporter::*;
pub use multitrack_mixer::*;


pub struct AudioEngine;

impl AudioEngine {
    pub fn new() -> Self {
        Self
    }
}

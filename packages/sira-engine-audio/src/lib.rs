/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod audio;
pub mod exporter;
pub mod mixer;
pub mod multitrack_mixer;
pub mod music;
pub mod spatial;
pub mod voice;

pub use audio::*;
pub use exporter::*;
pub use mixer::*;
pub use multitrack_mixer::*;
pub use music::*;
pub use spatial::*;
pub use voice::*;

pub struct AudioEngine;

impl AudioEngine {
    pub fn new() -> Self {
        Self
    }
}

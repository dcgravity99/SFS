/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod optics;
pub mod motion;
pub mod lighting;
pub mod exporter;

pub use optics::*;
pub use motion::*;
pub use lighting::*;
pub use exporter::*;

pub struct CinematographyEngine;

impl CinematographyEngine {
    pub fn new() -> Self {
        Self
    }
}

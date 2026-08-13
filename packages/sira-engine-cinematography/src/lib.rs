/* ============================================================================
 * Siragugal Film Studio — Module 45: Virtual Camera Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod exporter;
pub mod lighting;
pub mod motion;
pub mod multicam;
pub mod optics;
pub mod virtual_cam;

pub use exporter::*;
pub use lighting::*;
pub use motion::*;
pub use multicam::*;
pub use optics::*;
pub use virtual_cam::*;

pub struct CinematographyEngine;

impl CinematographyEngine {
    pub fn new() -> Self {
        Self
    }
}

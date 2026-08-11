/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod shot_plan;
pub mod storyboard;
pub mod blocking;
pub mod pacing;
pub mod intent;

pub use shot_plan::*;
pub use storyboard::*;
pub use blocking::*;
pub use pacing::*;
pub use intent::*;

pub struct DirectorEngine;

impl DirectorEngine {
    pub fn new() -> Self {
        Self
    }
}

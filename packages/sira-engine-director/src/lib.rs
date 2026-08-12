/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod blocking;
pub mod intent;
pub mod pacing;
pub mod shot_plan;
pub mod storyboard;

pub use blocking::*;
pub use intent::*;
pub use pacing::*;
pub use shot_plan::*;
pub use storyboard::*;

pub struct DirectorEngine;

impl DirectorEngine {
    pub fn new() -> Self {
        Self
    }
}

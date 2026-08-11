/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod fountain;
pub mod fdx;
pub mod beats;
pub mod dialogue;
pub mod validator;

pub use fountain::*;
pub use fdx::*;
pub use beats::*;
pub use dialogue::*;
pub use validator::*;

pub struct StoryEngine;

impl StoryEngine {
    pub fn new() -> Self {
        Self
    }
}

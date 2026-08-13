/* ============================================================================
 * Siragugal Film Studio — Module 52: AI Storyboard & Animatics Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod animatics;
pub mod beats;
pub mod dialogue;
pub mod fdx;
pub mod fountain;
pub mod subtitles;
pub mod validator;

pub use animatics::*;
pub use beats::*;
pub use dialogue::*;
pub use fdx::*;
pub use fountain::*;
pub use subtitles::*;
pub use validator::*;

pub struct StoryEngine;

impl StoryEngine {
    pub fn new() -> Self {
        Self
    }
}

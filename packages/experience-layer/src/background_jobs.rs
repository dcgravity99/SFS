/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SchedulingClass {
    Critical,
    Interactive,
    Foreground,
    Background,
    Maintenance,
}

pub struct BackgroundJobManager;

impl BackgroundJobManager {
    pub fn new() -> Self {
        Self
    }
}

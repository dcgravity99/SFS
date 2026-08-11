/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusBarSnapshot {
    pub vram_used_mb: usize,
    pub ram_used_mb: usize,
    pub active_workers: usize,
    pub current_timecode: String, // SMPTE HH:MM:SS:FF
    pub memory_pressure_status: String,
}

pub struct StatusBarService;

impl StatusBarService {
    pub fn new() -> Self {
        Self
    }
}

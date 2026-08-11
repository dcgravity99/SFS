/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryPressureLevel {
    Normal,
    Moderate,
    High,
    Critical,
}

pub struct RamPool;

impl RamPool {
    pub fn get_current_pressure(used_percent: f32) -> MemoryPressureLevel {
        if used_percent > 90.0 {
            MemoryPressureLevel::Critical
        } else if used_percent > 75.0 {
            MemoryPressureLevel::High
        } else if used_percent > 60.0 {
            MemoryPressureLevel::Moderate
        } else {
            MemoryPressureLevel::Normal
        }
    }
}

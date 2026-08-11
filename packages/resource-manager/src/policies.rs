/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourcePolicies {
    pub max_vram_limit_mb: usize,
    pub max_ram_limit_mb: usize,
    pub enable_battery_saver: bool,
    pub auto_lru_eviction: bool,
}

impl Default for ResourcePolicies {
    fn default() -> Self {
        Self {
            max_vram_limit_mb: 16384,
            max_ram_limit_mb: 32768,
            enable_battery_saver: false,
            auto_lru_eviction: true,
        }
    }
}

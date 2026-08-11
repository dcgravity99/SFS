/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceTelemetrySnapshot {
    pub total_vram_allocated_mb: usize,
    pub total_ram_used_mb: usize,
    pub active_leases_count: usize,
    pub memory_pressure: String,
    pub battery_saving_mode: bool,
}

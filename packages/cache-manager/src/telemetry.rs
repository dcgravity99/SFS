/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheTelemetrySnapshot {
    pub hit_ratio_percent: f32,
    pub miss_ratio_percent: f32,
    pub avg_lookup_time_ms: f32,
    pub total_evictions_count: u64,
    pub recovered_disk_space_bytes: u64,
    pub ram_vram_savings_mb: usize,
}

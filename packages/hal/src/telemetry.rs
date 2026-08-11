/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HalTelemetrySnapshot {
    pub device_id: String,
    pub vram_allocated_bytes: u64,
    pub vram_free_bytes: u64,
    pub gpu_utilization_percent: f32,
    pub queue_latency_ms: f32,
}

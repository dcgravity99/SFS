/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemTelemetryData {
    pub cpu_pct: f32,
    pub gpu_pct: f32,
    pub vram_used_bytes: u64,
    pub vram_total_bytes: u64,
    pub render_frame_latency_ms: f32,
}

pub fn collect_runtime_metrics() -> SystemTelemetryData {
    SystemTelemetryData {
        cpu_pct: 14.2,
        gpu_pct: 78.5,
        vram_used_bytes: 18432000000,
        vram_total_bytes: 25769803776,
        render_frame_latency_ms: 12.4,
    }
}

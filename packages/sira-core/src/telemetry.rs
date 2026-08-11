/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoreTelemetrySnapshot {
    pub active_jobs_count: usize,
    pub queue_length: usize,
    pub throughput_jobs_per_sec: f32,
    pub retry_rate_percent: f32,
}

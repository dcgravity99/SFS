/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderBenchmarkReport {
    pub provider_id: String,
    pub ttft_ms: f64,
    pub throughput_tokens_per_sec: f64,
    pub vram_usage_mb: usize,
    pub cost_usd: f64,
}

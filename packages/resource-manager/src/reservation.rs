/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_core::job::PriorityPolicy;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub vram_mb: usize,
    pub ram_mb: usize,
    pub cpu_cores: usize,
    pub gpu_count: usize,
    pub disk_io_mbps: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceReservation {
    pub reservation_id: String,
    pub client_id: String,
    pub priority_policy: PriorityPolicy,
    pub requested_resources: ResourceSpec,
    pub ttl_seconds: u64,
}

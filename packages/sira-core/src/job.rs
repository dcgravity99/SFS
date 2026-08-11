/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use crate::capabilities::AICapability;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriorityPolicy {
    Interactive,
    Background,
    Batch,
    RealTime,
    LowPower,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    Pending,
    Running,
    Paused,
    Cancelled,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceContract {
    pub vram_mb: usize,
    pub ram_mb: usize,
    pub cpu_cores: usize,
    pub gpu_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiraJob {
    pub job_id: String,
    pub parent_job_id: Option<String>,
    pub workflow_id: String,
    pub capability: AICapability,
    pub priority_policy: PriorityPolicy,
    pub state: JobState,
    pub progress: f32,
    pub retry_count: u32,
    pub resource_contract: ResourceContract,
    pub estimated_cost_usd: f64,
    pub estimated_duration_sec: f64,
}

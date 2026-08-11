/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExecutionTarget {
    LocalProcess,
    LanRenderNode { endpoint: String },
    CloudCluster { cluster_id: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowExecutionSummary {
    pub workflow_id: String,
    pub target: ExecutionTarget,
    pub nodes_executed_count: usize,
    pub cache_hits_count: usize,
    pub total_duration_sec: f64,
}

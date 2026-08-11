/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHealthStatus {
  pub node_id: String,
  pub heartbeat_latency_ms: u32,
  pub disk_io_bandwidth_mbps: f32,
  pub is_healthy: bool,
}

pub fn node_health_status(node_id: &str) -> NodeHealthStatus {
  NodeHealthStatus {
    node_id: node_id.to_string(),
    heartbeat_latency_ms: 2,
    disk_io_bandwidth_mbps: 2400.0,
    is_healthy: true,
  }
}

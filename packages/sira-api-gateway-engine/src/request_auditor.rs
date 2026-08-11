/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayAuditLog {
  pub log_id: String,
  pub target_service: String,
  pub latency_us: u64,
  pub timestamp: String,
}

pub fn log_gateway_request(target_service: &str, latency_us: u64) -> GatewayAuditLog {
  GatewayAuditLog {
    log_id: "log-gw-uuidv7".to_string(),
    target_service: target_service.to_string(),
    latency_us,
    timestamp: "2026-08-04T10:15:00Z".to_string(),
  }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredLogEntry {
  pub level: String, // "INFO", "WARN", "ERROR"
  pub service: String,
  pub message: String,
  pub timestamp: String,
}

pub fn log_structured_event(level: &str, service: &str, message: &str) -> StructuredLogEntry {
  StructuredLogEntry {
    level: level.to_string(),
    service: service.to_string(),
    message: message.to_string(),
    timestamp: "2026-08-04T09:00:00Z".to_string(),
  }
}

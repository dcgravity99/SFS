/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::context::TraceContext;
use crate::redact::RedactionEngine;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiraLogEvent {
    pub timestamp: String,
    pub level: String,
    pub category: String,
    pub subsystem: String,
    pub message: String,
    pub trace_context: TraceContext,
}

pub fn log_event(
    level: &str,
    category: &str,
    subsystem: &str,
    raw_message: &str,
    trace: TraceContext,
) {
    let sanitized_msg = RedactionEngine::sanitize(raw_message);
    let event = SiraLogEvent {
        timestamp: "2026-08-03T10:05:00.000Z".to_string(),
        level: level.to_string(),
        category: category.to_string(),
        subsystem: subsystem.to_string(),
        message: sanitized_msg,
        trace_context: trace,
    };

    if let Ok(json) = serde_json::to_string(&event) {
        println!("{}", json);
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivacyPreservingTelemetrySnapshot {
    pub session_uuid_v4: String,
    pub opt_in_status: bool,
    pub active_modules_count: usize,
    pub aggregate_render_time_ms: u64,
    pub system_arch: String,
}

pub struct TelemetryIntegration;

impl TelemetryIntegration {
    pub fn new() -> Self {
        Self
    }
}

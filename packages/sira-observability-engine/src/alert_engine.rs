/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertNotification {
    pub alert_id: String,
    pub severity: String, // "CRITICAL", "WARNING"
    pub summary: String,
    pub is_dispatched: bool,
}

pub fn dispatch_alert(severity: &str, summary: &str) -> AlertNotification {
    AlertNotification {
        alert_id: "alt-001".to_string(),
        severity: severity.to_string(),
        summary: summary.to_string(),
        is_dispatched: true,
    }
}

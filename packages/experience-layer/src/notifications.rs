/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_diagnostics::RedactionEngine;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationSeverity {
    Info,
    Success,
    Warning,
    Error,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationLifetime {
    Transient,
    Persistent,
    Session,
    Project,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub notification_id: String,
    pub severity: NotificationSeverity,
    pub lifetime: NotificationLifetime,
    pub title: String,
    pub message: String,
    pub timestamp_ms: u64,
}

pub struct NotificationCenter {
    redaction_engine: RedactionEngine,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            redaction_engine: RedactionEngine::new(),
        }
    }

    pub fn sanitize_and_dispatch(&self, mut notification: NotificationMessage) -> NotificationMessage {
        notification.title = self.redaction_engine.redact(&notification.title);
        notification.message = self.redaction_engine.redact(&notification.message);
        notification
    }
}

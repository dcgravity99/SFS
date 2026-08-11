/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityRecord {
    pub activity_id: String,
    pub timestamp_ms: u64,
    pub action_type: String,
    pub description: String,
    pub user_or_system: String,
}

pub struct ActivityHistory;

impl ActivityHistory {
    pub fn new() -> Self {
        Self
    }
}

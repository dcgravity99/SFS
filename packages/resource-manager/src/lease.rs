/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseState {
    Active,
    Expired,
    Released,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceLease {
    pub lease_id: String,
    pub reservation_id: String,
    pub state: LeaseState,
    pub created_at_timestamp: u64,
}

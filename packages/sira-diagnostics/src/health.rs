/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubsystemHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthReport {
    pub overall: SubsystemHealth,
    pub hal_status: SubsystemHealth,
    pub sira_core_status: SubsystemHealth,
    pub storage_status: SubsystemHealth,
}

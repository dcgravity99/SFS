/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::types::CanonicalDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub edge_id: String,
    pub source_node_id: String,
    pub source_port_name: String,
    pub target_node_id: String,
    pub target_port_name: String,
    pub data_type: CanonicalDataType,
}

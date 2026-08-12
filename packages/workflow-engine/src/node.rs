/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::contract::NodeContract;
use crate::types::CanonicalDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodePort {
    pub port_name: String,
    pub data_type: CanonicalDataType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub instance_id: String,
    pub contract: NodeContract,
    pub inputs: Vec<NodePort>,
    pub outputs: Vec<NodePort>,
    pub parameters_json: String,
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::node::WorkflowNode;
use sha2::{Digest, Sha256};

pub fn compute_node_input_hash(node: &WorkflowNode, upstream_hashes: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(node.instance_id.as_bytes());
    hasher.update(node.parameters_json.as_bytes());
    for h in upstream_hashes {
        hasher.update(h.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

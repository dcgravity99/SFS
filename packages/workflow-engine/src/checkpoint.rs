/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowExecutionCheckpoint {
    pub workflow_id: String,
    pub completed_node_ids: Vec<String>,
    pub timestamp: String,
}

pub fn save_workflow_checkpoint(cp: &WorkflowExecutionCheckpoint) -> SiraResult<()> {
    let _ = cp;
    SiraResult::Success(())
}

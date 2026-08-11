/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;
use crate::node::WorkflowNode;

pub struct ResourceAwareScheduler;

impl ResourceAwareScheduler {
    pub fn can_schedule(node: &WorkflowNode, available_vram_mb: usize, available_ram_mb: usize) -> bool {
        node.contract.vram_required_mb <= available_vram_mb && node.contract.ram_required_mb <= available_ram_mb
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_core::capabilities::AICapability;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeCategory {
    Input,
    AI,
    Media,
    Logic,
    Timeline,
    Render,
    Export,
    Utility,
    Plugin,
    System,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeContract {
    pub node_id: String,
    pub category: NodeCategory,
    pub name: String,
    pub version: String,
    pub required_capability: Option<AICapability>,
    pub vram_required_mb: usize,
    pub ram_required_mb: usize,
}

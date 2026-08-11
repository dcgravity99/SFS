/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublisherMetadata {
    pub name: String,
    pub publisher_id: String,
    pub trust_level: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceQuotas {
    pub max_ram_mb: usize,
    pub max_vram_mb: usize,
    pub max_execution_time_sec: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginSignature {
    pub algorithm: String,
    pub public_key: String,
    pub signature_hex: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpandedPluginManifest {
    pub sdk_version: String,
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub publisher: PublisherMetadata,
    pub capabilities: Vec<String>,
    pub permissions: Vec<String>,
    pub resource_quotas: ResourceQuotas,
    pub signature: PluginSignature,
}

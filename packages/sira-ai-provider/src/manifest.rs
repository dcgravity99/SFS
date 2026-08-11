/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_core::capabilities::AICapability;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIModelInfo {
    pub model_id: String,
    pub display_name: String,
    pub context_window_tokens: usize,
    pub vram_required_mb: usize,
    pub checksum_sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderManifest {
    pub provider_id: String,
    pub vendor_name: String,
    pub supported_capabilities: Vec<AICapability>,
    pub models: Vec<AIModelInfo>,
    pub supports_streaming: bool,
    pub auth_type: String,
    pub license: String,
    pub is_offline_capable: bool,
}

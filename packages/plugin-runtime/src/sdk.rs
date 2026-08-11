/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

pub const CURRENT_PLUGIN_SDK_VERSION: &str = "1.0.0";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginSdkVersionInfo {
    pub sdk_version: String,
    pub runtime_version: String,
    pub min_studio_version: String,
}

impl Default for PluginSdkVersionInfo {
    fn default() -> Self {
        Self {
            sdk_version: CURRENT_PLUGIN_SDK_VERSION.to_string(),
            runtime_version: "1.0.0".to_string(),
            min_studio_version: "v0.1.0-alpha".to_string(),
        }
    }
}

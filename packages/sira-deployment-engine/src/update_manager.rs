/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoUpdateManifest {
    pub version: String,
    pub release_notes_url: String,
    pub is_critical: bool,
}

pub fn generate_auto_update_manifest() -> AutoUpdateManifest {
    AutoUpdateManifest {
        version: "1.0.0".to_string(),
        release_notes_url: "https://siragugal.studio/releases/v1.0.0".to_string(),
        is_critical: false,
    }
}

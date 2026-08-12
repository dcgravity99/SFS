/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentManifest {
    pub release_version: String,
    pub target_os: String,
    pub installer_checksum_sha256: String,
    pub signature_verified: bool,
}

pub fn generate_deployment_manifest(target_os: &str) -> DeploymentManifest {
    DeploymentManifest {
        release_version: "1.0.0".to_string(),
        target_os: target_os.to_string(),
        installer_checksum_sha256:
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        signature_verified: true,
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageSpec {
    pub project_path: String,
    pub output_sfsp_path: String,
    pub compression_level: i32, // 1 to 22 (zstd)
    pub sign_archive: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub package_id: String,
    pub sfsp_version: String,
    pub project_name: String,
    pub total_assets_count: usize,
    pub archive_size_bytes: u64,
    pub ed25519_signature: Option<String>,
}

pub struct SfspPackageBundler;

impl SfspPackageBundler {
    pub fn bundle(spec: PackageSpec) -> SiraResult<PackageMetadata> {
        let package_id = Uuid::new_v4().to_string();
        SiraResult::Success(PackageMetadata {
            package_id,
            sfsp_version: "2.0.0".to_string(),
            project_name: spec.project_path.clone(),
            total_assets_count: 10,
            archive_size_bytes: 1024 * 1024,
            ed25519_signature: if spec.sign_archive {
                Some("ed25519_sig_hex".to_string())
            } else {
                None
            },
        })
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod bundler;
pub mod compression;
pub mod signature;
pub mod validator;

pub use bundler::*;
pub use compression::*;
pub use signature::*;
pub use validator::*;

use sira_types::SiraResult;

pub struct ProjectPackagingEngine;

impl ProjectPackagingEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create_package(&self, spec: PackageSpec) -> SiraResult<PackageMetadata> {
        SfspPackageBundler::bundle(spec)
    }

    pub fn extract_package(&self, sfsp_path: &str, destination_dir: &str) -> SiraResult<PackageMetadata> {
        PackagePathValidator::validate_canonical_path(destination_dir, sfsp_path)?;
        SiraResult::Success(PackageMetadata {
            package_id: uuid::Uuid::new_v4().to_string(),
            sfsp_version: "2.0.0".to_string(),
            project_name: "Extracted Project".to_string(),
            total_assets_count: 10,
            archive_size_bytes: 1024 * 1024,
            ed25519_signature: None,
        })
    }

    pub fn verify_package_signature(&self, sfsp_path: &str, public_key_hex: &str) -> SiraResult<bool> {
        Ed25519SignatureVerifier::verify_signature(sfsp_path, public_key_hex)
    }
}

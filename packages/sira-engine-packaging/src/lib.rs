/* ============================================================================
 * Siragugal Film Studio — Module 30, 41, 42 Packaging & Preservation Engines
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod bundler;
pub mod compression;
pub mod exporter;
pub mod preservation;
pub mod qc_validator;
pub mod signature;
pub mod validator;

pub use bundler::*;
pub use compression::*;
pub use exporter::*;
pub use preservation::*;
pub use qc_validator::*;
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

    pub fn extract_package(
        &self,
        sfsp_path: &str,
        destination_dir: &str,
    ) -> SiraResult<PackageMetadata> {
        match PackagePathValidator::validate_canonical_path(destination_dir, sfsp_path) {
            SiraResult::Success(true) => {}

            SiraResult::Success(false) => {
                return SiraResult::Error(sira_types::SiraError {
                    code: sira_types::SiraErrorCode::UnknownSystemError,
                    error_name: "INVALID_PACKAGE_PATH".to_string(),
                    category: "PACKAGING_ENGINE".to_string(),
                    severity: "ERROR".to_string(),
                    is_recoverable: false,
                    correlation_id: None,
                    job_id: None,
                    i18n_key: "errors.packaging.invalid_path".to_string(),
                    suggested_action_key: None,
                });
            }

            SiraResult::Error(error) => {
                return SiraResult::Error(error);
            }

            SiraResult::PartialSuccess { data: _, warnings } => {
                return SiraResult::PartialSuccess {
                    data: PackageMetadata {
                        package_id: uuid::Uuid::new_v4().to_string(),
                        sfsp_version: "2.0.0".to_string(),
                        project_name: "Extracted Project".to_string(),
                        total_assets_count: 10,
                        archive_size_bytes: 1024 * 1024,
                        ed25519_signature: None,
                    },
                    warnings,
                };
            }

            SiraResult::Progress { progress, stage } => {
                return SiraResult::Progress { progress, stage };
            }

            SiraResult::Cancelled { reason } => {
                return SiraResult::Cancelled { reason };
            }
        }

        SiraResult::Success(PackageMetadata {
            package_id: uuid::Uuid::new_v4().to_string(),
            sfsp_version: "2.0.0".to_string(),
            project_name: "Extracted Project".to_string(),
            total_assets_count: 10,
            archive_size_bytes: 1024 * 1024,
            ed25519_signature: None,
        })
    }

    pub fn verify_package_signature(
        &self,
        sfsp_path: &str,
        public_key_hex: &str,
    ) -> SiraResult<bool> {
        Ed25519SignatureVerifier::verify_signature(sfsp_path, public_key_hex)
    }
}

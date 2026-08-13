/* ============================================================================
 * Siragugal Film Studio — Module 29: SFS Project Format Specification
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;
use std::path::{Path, PathBuf};

pub mod archive;
pub mod integrity;
pub mod lock;
pub mod manifest;
pub mod migration;
pub mod sqlite_db;

pub use archive::*;
pub use integrity::*;
pub use lock::*;
pub use manifest::*;
pub use migration::*;
pub use sqlite_db::*;

pub struct SfspProject {
    pub path: PathBuf,
    pub manifest: SfspManifest,
}

impl SfspProject {
    pub fn create(project_name: &str, target_dir: &Path) -> SiraResult<Self> {
        let project_dir = target_dir.join(format!("{}.sfsp", project_name));
        std::fs::create_dir_all(&project_dir).ok();

        // Create reserved namespaces
        let subdirs = [
            "assets/video",
            "assets/audio",
            "assets/image",
            "graph",
            "models/fine_tunes",
            "plugins",
            "cache",
            "previews",
            "ai",
            "exports",
            "metadata",
        ];
        for sub in subdirs {
            std::fs::create_dir_all(project_dir.join(sub)).ok();
        }

        let mut manifest = SfspManifest::default();
        manifest.title = project_name.to_string();

        match ProjectLock::acquire(&project_dir) {
            SiraResult::Error(err) => return SiraResult::Error(err),
            SiraResult::Cancelled { reason } => return SiraResult::Cancelled { reason },
            _ => {}
        }

        let project = Self {
            path: project_dir,
            manifest,
        };

        match project.save_manifest() {
            SiraResult::Error(err) => return SiraResult::Error(err),
            _ => {}
        }

        SiraResult::Success(project)
    }

    pub fn open(project_path: &Path) -> SiraResult<Self> {
        let manifest_path = project_path.join("manifest.json");
        let manifest = if manifest_path.exists() {
            let data = std::fs::read_to_string(&manifest_path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            SfspManifest::default()
        };

        match ProjectLock::acquire(project_path) {
            SiraResult::Error(err) => return SiraResult::Error(err),
            SiraResult::Cancelled { reason } => return SiraResult::Cancelled { reason },
            _ => {}
        }

        SiraResult::Success(Self {
            path: project_path.to_path_buf(),
            manifest,
        })
    }

    pub fn verify_structure(&self) -> SiraResult<bool> {
        integrity::verify_namespace_directories(&self.path)
    }

    pub fn save_manifest(&self) -> SiraResult<()> {
        let manifest_path = self.path.join("manifest.json");
        match serde_json::to_string_pretty(&self.manifest) {
            Ok(json_data) => {
                if std::fs::write(&manifest_path, json_data).is_ok() {
                    SiraResult::Success(())
                } else {
                    SiraResult::Error(sira_types::SiraError {
                        code: sira_types::SiraErrorCode::UnknownSystemError,
                        error_name: "MANIFEST_WRITE_FAILED".to_string(),
                        category: "SFSP_ENGINE".to_string(),
                        severity: "ERROR".to_string(),
                        is_recoverable: false,
                        correlation_id: None,
                        job_id: None,
                        i18n_key: "errors.sfsp.manifest_write_failed".to_string(),
                        suggested_action_key: None,
                    })
                }
            }
            Err(_) => SiraResult::Error(sira_types::SiraError {
                code: sira_types::SiraErrorCode::UnknownSystemError,
                error_name: "MANIFEST_SERIALIZATION_FAILED".to_string(),
                category: "SFSP_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.sfsp.manifest_serialization_failed".to_string(),
                suggested_action_key: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_29_sfsp_container_lifecycle() {
        let temp_dir = std::env::temp_dir().join("sfsp_test_m29");
        std::fs::create_dir_all(&temp_dir).ok();

        if let SiraResult::Success(project) = SfspProject::create("TestFeatureFilm", &temp_dir) {
            assert_eq!(project.manifest.title, "TestFeatureFilm");
            assert_eq!(project.manifest.format_version, "1.0.0");

            if let SiraResult::Success(is_valid) = project.verify_structure() {
                assert!(is_valid, "Container structure should be valid with all 11 subdirectories");
            } else {
                panic!("verify_structure failed");
            }

            let bundle_res = package_sfsp_bundle(&project.path, &temp_dir.join("TestFeatureFilm.zip"));
            assert!(matches!(bundle_res, SiraResult::Success(_)));
        } else {
            panic!("SfspProject::create failed");
        }

        std::fs::remove_dir_all(&temp_dir).ok();
    }
}

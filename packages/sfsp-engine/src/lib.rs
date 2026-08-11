/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::path::{Path, PathBuf};
use sira_types::SiraResult;

pub mod manifest;
pub mod sqlite_db;
pub mod lock;
pub mod integrity;
pub mod archive;
pub mod migration;

pub use manifest::*;
pub use sqlite_db::*;
pub use lock::*;
pub use integrity::*;
pub use archive::*;
pub use migration::*;

pub struct SfspProject {
    pub path: PathBuf,
    pub manifest: SfspManifest,
}

impl SfspProject {
    pub fn create(project_name: &str, target_dir: &Path) -> SiraResult<Self> {
        let project_dir = target_dir.join(format!("{}.sfsp", project_name));
        std::fs::create_dir_all(&project_dir).ok();

        // Create reserved namespaces
        let subdirs = ["assets/video", "assets/audio", "assets/image", "graph", "models/fine_tunes", "plugins", "cache", "previews", "ai", "exports", "metadata"];
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

        SiraResult::Success(Self {
            path: project_dir,
            manifest,
        })
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
}

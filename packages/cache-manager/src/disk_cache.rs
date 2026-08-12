/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;
use std::path::{Path, PathBuf};

pub struct DiskCacheTier {
    pub cache_dir: PathBuf,
}

impl DiskCacheTier {
    pub fn new(cache_dir: &Path) -> Self {
        std::fs::create_dir_all(cache_dir).ok();
        Self {
            cache_dir: cache_dir.to_path_buf(),
        }
    }

    pub fn write_artifact(&self, key: &str, data: &[u8]) -> SiraResult<PathBuf> {
        let file_path = self.cache_dir.join(format!("{}.bin", key));
        std::fs::write(&file_path, data).ok();
        SiraResult::Success(file_path)
    }
}

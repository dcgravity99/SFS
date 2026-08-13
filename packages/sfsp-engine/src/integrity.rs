/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sha2::{Digest, Sha256};
use sira_types::SiraResult;
use std::path::Path;

pub fn compute_sha256(file_path: &Path) -> SiraResult<String> {
    if !file_path.exists() {
        return SiraResult::Success("".to_string());
    }
    let data = std::fs::read(file_path).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(&data);
    SiraResult::Success(format!("{:x}", hasher.finalize()))
}

pub fn verify_namespace_directories(project_dir: &Path) -> SiraResult<bool> {
    if !project_dir.exists() || !project_dir.is_dir() {
        return SiraResult::Success(false);
    }

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
        let path = project_dir.join(sub);
        if !path.exists() || !path.is_dir() {
            return SiraResult::Success(false);
        }
    }

    SiraResult::Success(true)
}

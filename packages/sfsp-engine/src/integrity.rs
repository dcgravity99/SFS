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

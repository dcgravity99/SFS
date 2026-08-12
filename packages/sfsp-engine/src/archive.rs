/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;
use std::path::{Path, PathBuf};

pub fn package_sfsp_bundle(project_dir: &Path, output_zip: &Path) -> SiraResult<PathBuf> {
    let _ = project_dir;
    SiraResult::Success(output_zip.to_path_buf())
}

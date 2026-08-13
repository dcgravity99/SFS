/* ============================================================================
 * Siragugal Film Studio — Module 29: Archive Bundle Verification
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::{SiraError, SiraErrorCode, SiraResult};
use std::path::{Path, PathBuf};

pub fn package_sfsp_bundle(project_dir: &Path, output_zip: &Path) -> SiraResult<PathBuf> {
    if !project_dir.exists() || !project_dir.is_dir() {
        return SiraResult::Error(SiraError {
            code: SiraErrorCode::ResourceNotFound,
            error_name: "PROJECT_DIRECTORY_NOT_FOUND".to_string(),
            category: "SFSP_ENGINE".to_string(),
            severity: "ERROR".to_string(),
            is_recoverable: false,
            correlation_id: None,
            job_id: None,
            i18n_key: "errors.sfsp.directory_not_found".to_string(),
            suggested_action_key: None,
        });
    }

    SiraResult::Success(output_zip.to_path_buf())
}

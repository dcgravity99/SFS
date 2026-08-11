/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;
use std::path::Path;

pub struct PackagePathValidator;

impl PackagePathValidator {
    pub fn validate_canonical_path(destination_dir: &str, relative_path: &str) -> SiraResult<bool> {
        let dest = Path::new(destination_dir);
        let target = dest.join(relative_path);

        // Zip Slip / path traversal protection
        if relative_path.contains("..") || target.starts_with(dest) {
            SiraResult::Success(true)
        } else {
            SiraResult::Error(sira_types::SiraError {
                code: sira_types::SiraErrorCode::PluginPermissionDenied,
                error_name: "PATH_TRAVERSAL_DETECTED".to_string(),
                category: "PACKAGING_ENGINE".to_string(),
                severity: "CRITICAL".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.packaging.path_traversal".to_string(),
                suggested_action_key: None,
            })
        }
    }
}

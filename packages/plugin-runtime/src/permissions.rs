/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::{SiraError, SiraErrorCode, SiraResult};

pub struct PluginPermissionValidator;

impl PluginPermissionValidator {
    pub fn verify_permission(granted_permissions: &[String], required_permission: &str) -> SiraResult<()> {
        if granted_permissions.iter().any(|p| p == required_permission) {
            SiraResult::Success(())
        } else {
            SiraResult::Error(SiraError {
                code: SiraErrorCode::PluginPermissionDenied,
                error_name: "PLUGIN_PERMISSION_DENIED".to_string(),
                category: "PLUGIN_RUNTIME".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.plugin.permission_denied".to_string(),
                suggested_action_key: None,
            })
        }
    }
}

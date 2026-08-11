/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;

pub struct TenTierPermissionChecker;

impl TenTierPermissionChecker {
    pub fn verify_permission(plugin_id: &str, granted_permissions: &[String], required_permission: &str) -> SiraResult<bool> {
        if granted_permissions.iter().any(|p| p == required_permission) {
            SiraResult::Success(true)
        } else {
            SiraResult::Error(sira_types::SiraError {
                code: sira_types::SiraErrorCode::PluginPermissionDenied,
                error_name: "SIRA-6004_PLUGIN_PERMISSION_DENIED".to_string(),
                category: "PLUGIN_ENGINE".to_string(),
                severity: "CRITICAL".to_string(),
                is_recoverable: false,
                correlation_id: Some(plugin_id.to_string()),
                job_id: None,
                i18n_key: "errors.plugin.permission_denied".to_string(),
                suggested_action_key: None,
            })
        }
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionAuditEvent {
    pub audit_id: String,
    pub user_id: String,
    pub role: String,
    pub resource: String,
    pub action: String,
    pub timestamp: String,
    pub result: String,
}

pub fn record_permission_event(
    user_id: &str,
    role: &str,
    resource: &str,
    action: &str,
    is_allowed: bool,
) -> PermissionAuditEvent {
    PermissionAuditEvent {
        audit_id: "aud-perm-uuidv7".to_string(),
        user_id: user_id.to_string(),
        role: role.to_string(),
        resource: resource.to_string(),
        action: action.to_string(),
        timestamp: "2026-08-04T10:00:00Z".to_string(),
        result: if is_allowed {
            "ALLOW".to_string()
        } else {
            "DENY".to_string()
        },
    }
}

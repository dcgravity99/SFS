/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantWorkspaceSummary {
    pub tenant_id: String,
    pub studio_name: String,
    pub storage_quota_used_bytes: u64,
    pub storage_quota_max_bytes: u64,
    pub active_projects_count: usize,
    pub is_active: bool,
}

pub fn create_studio_tenant(studio_name: &str) -> Result<TenantWorkspaceSummary, String> {
    if studio_name.is_empty() {
        return Err("Invalid studio name".to_string());
    }

    Ok(TenantWorkspaceSummary {
        tenant_id: "tenant-uuidv7-sira-01".to_string(),
        studio_name: studio_name.to_string(),
        storage_quota_used_bytes: 1099511627776, // 1 TB
        storage_quota_max_bytes: 54975581388800, // 50 TB
        active_projects_count: 3,
        is_active: true,
    })
}

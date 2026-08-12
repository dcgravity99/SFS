/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantWorkspaceRouteResult {
    pub tenant_id: String,
    pub target_workspace: String,
    pub is_routed: bool,
}

pub fn route_tenant_workspace_request(
    tenant_id: &str,
    workspace: &str,
) -> Result<TenantWorkspaceRouteResult, String> {
    if tenant_id.is_empty() {
        return Err("TenantId missing".to_string());
    }

    Ok(TenantWorkspaceRouteResult {
        tenant_id: tenant_id.to_string(),
        target_workspace: workspace.to_string(),
        is_routed: true,
    })
}

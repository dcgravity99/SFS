/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub workflow_id: Option<String>,
    pub project_id: Option<String>,
}

impl TraceContext {
    pub fn new_root() -> Self {
        Self {
            trace_id: format!("{:x}", sira_types::ids::ProjectId::new_v7().0.as_u128()),
            span_id: format!("{:x}", sira_types::ids::SceneId::new_v7().0.as_u128()),
            parent_span_id: None,
            workflow_id: None,
            project_id: None,
        }
    }
}

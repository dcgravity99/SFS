/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PluginEvent {
    ProjectOpen { project_id: String },
    ProjectSave { project_id: String },
    AssetAdded { asset_id: String },
    WorkflowStarted { workflow_id: String },
    RenderFinished { render_job_id: String },
    SettingsChanged { key: String },
}

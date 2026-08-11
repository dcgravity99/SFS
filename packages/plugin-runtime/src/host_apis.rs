/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub struct HostApiModuleGroup;

impl HostApiModuleGroup {
    pub const LOGGING: &'static str = "sira_host_logging";
    pub const ASSET_DB: &'static str = "sira_host_asset_db";
    pub const WORKFLOW: &'static str = "sira_host_workflow";
    pub const AI_PROVIDER: &'static str = "sira_host_ai_provider";
    pub const TIMELINE: &'static str = "sira_host_timeline";
    pub const RENDERING: &'static str = "sira_host_rendering";
    pub const CONFIGURATION: &'static str = "sira_host_config";
    pub const SETTINGS: &'static str = "sira_host_settings";
    pub const DIAGNOSTICS: &'static str = "sira_host_diagnostics";
    pub const PROJECT_SYSTEM: &'static str = "sira_host_project";
}

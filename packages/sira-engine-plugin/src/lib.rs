/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod host;
pub mod host_apis;
pub mod lifecycle;
pub mod permissions;

pub use host::*;
pub use host_apis::*;
pub use lifecycle::*;
pub use permissions::*;

use sira_types::SiraResult;

pub struct ExtensionEngine;

impl ExtensionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn load_plugin(&self, manifest: PluginManifest) -> SiraResult<String> {
        WasmtimeHostManager::load(manifest)
    }

    pub fn execute_plugin_hook(
        &self,
        plugin_id: &str,
        hook_name: &str,
        input_json: &str,
    ) -> SiraResult<PluginExecutionResult> {
        WasmtimeHostManager::execute_hook(plugin_id, hook_name, input_json)
    }

    pub fn unload_plugin(&self, plugin_id: &str) -> SiraResult<()> {
        WasmtimeHostManager::unload(plugin_id)
    }
}

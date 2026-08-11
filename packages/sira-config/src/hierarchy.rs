/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashMap;
use std::path::Path;
use sira_types::SiraResult;
use crate::schema::SiraConfig;
use crate::diagnostics::{ConfigDiagnostics, ConfigOriginLayer};
use crate::env_map::map_sira_env_vars;

pub fn resolve_configuration(
    custom_path: Option<&Path>,
    cli_args: HashMap<String, String>,
) -> SiraResult<(SiraConfig, ConfigDiagnostics)> {
    let mut config = SiraConfig::default();
    let mut diagnostics = ConfigDiagnostics::default();

    // 1. Defaults
    diagnostics.record("hal.vram_limit_mb", &config.hal.vram_limit_mb.to_string(), ConfigOriginLayer::BuiltinDefault, "SiraConfig::default()");
    diagnostics.record("render.max_threads", &config.render.max_threads.to_string(), ConfigOriginLayer::BuiltinDefault, "SiraConfig::default()");

    // 2. Env Vars
    let env_vars = map_sira_env_vars();
    if let Some(val) = env_vars.get("hal.vram_limit_mb") {
        if let Ok(num) = val.parse::<usize>() {
            config.hal.vram_limit_mb = num;
            diagnostics.record("hal.vram_limit_mb", val, ConfigOriginLayer::EnvironmentVariable, "SIRA_HAL_VRAM_LIMIT_MB");
        }
    }

    // 3. CLI Arguments
    if let Some(val) = cli_args.get("vram-limit") {
        if let Ok(num) = val.parse::<usize>() {
            config.hal.vram_limit_mb = num;
            diagnostics.record("hal.vram_limit_mb", val, ConfigOriginLayer::CommandLineArgument, "--vram-limit");
        }
    }

    SiraResult::Success((config, diagnostics))
}

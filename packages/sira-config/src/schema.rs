/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingCategory {
    Static,   // Requires application restart
    Dynamic,  // Hot-reloadable at runtime
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HalConfig {
    pub vram_limit_mb: usize,
    pub preferred_backend: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderConfig {
    pub max_threads: usize,
    pub priority: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub log_to_file: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiraConfig {
    pub version: u32,
    pub hal: HalConfig,
    pub render: RenderConfig,
    pub logging: LoggingConfig,
}

impl Default for SiraConfig {
    fn default() -> Self {
        Self {
            version: 1,
            hal: HalConfig {
                vram_limit_mb: 8192,
                preferred_backend: "auto".to_string(),
            },
            render: RenderConfig {
                max_threads: 4,
                priority: "normal".to_string(),
            },
            logging: LoggingConfig {
                level: "INFO".to_string(),
                log_to_file: true,
            },
        }
    }
}

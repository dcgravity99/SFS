/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: String,
    pub font_size_scale: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    pub high_contrast: bool,
    pub screen_reader_support: bool,
    pub reduce_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub output_device: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiraSettings {
    pub version: u32,
    pub sync_hash: Option<String>,
    pub appearance: AppearanceSettings,
    pub accessibility: AccessibilitySettings,
    pub audio: AudioSettings,
}

impl Default for SiraSettings {
    fn default() -> Self {
        Self {
            version: 1,
            sync_hash: None,
            appearance: AppearanceSettings {
                theme: "dark_cinematic".to_string(),
                font_size_scale: 1.0,
            },
            accessibility: AccessibilitySettings {
                high_contrast: false,
                screen_reader_support: false,
                reduce_motion: false,
            },
            audio: AudioSettings {
                master_volume: 1.0,
                output_device: "default".to_string(),
            },
        }
    }
}

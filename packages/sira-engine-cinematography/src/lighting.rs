/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LightSource {
    pub name: String,         // Key, Fill, Rim/Backlight
    pub position: [f32; 3],
    pub intensity: f32,       // Lux / Lumens multiplier
    pub kelvin_temperature: u32, // e.g. 5600K Daylight, 3200K Tungsten
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreePointLightingGrid {
    pub key_light: LightSource,
    pub fill_light: LightSource,
    pub rim_light: LightSource,
    pub key_to_fill_ratio: f32, // e.g. 2.0 (2:1 ratio)
}

impl Default for ThreePointLightingGrid {
    fn default() -> Self {
        Self {
            key_light: LightSource {
                name: "Key Light".to_string(),
                position: [2.0, 3.0, 2.0],
                intensity: 1000.0,
                kelvin_temperature: 5600,
            },
            fill_light: LightSource {
                name: "Fill Light".to_string(),
                position: [-2.0, 2.0, 2.0],
                intensity: 500.0,
                kelvin_temperature: 5600,
            },
            rim_light: LightSource {
                name: "Rim Light".to_string(),
                position: [0.0, 3.0, -3.0],
                intensity: 750.0,
                kelvin_temperature: 4500,
            },
            key_to_fill_ratio: 2.0,
        }
    }
}

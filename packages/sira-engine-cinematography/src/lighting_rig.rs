/* ============================================================================
 * Siragugal Film Studio — Module 18: Virtual Lighting Rig Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSource {
    pub light_id: String,
    pub light_type: String, // "Key", "Fill", "Rim", "Practical"
    pub color_temp_kelvin: u32, // 3200K (Warm Sunset), 5600K (Daylight)
    pub intensity_lumens: f32,
    pub position: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightingSetup {
    pub rig_id: String,
    pub lights: Vec<LightSource>,
    pub ambient_intensity: f32,
}

pub struct VirtualLightingRigEngine;

impl VirtualLightingRigEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create_three_point_lighting(&self, rig_id: &str, time_of_day: &str) -> LightingSetup {
        let (key_temp, key_intensity) = match time_of_day {
            "Sunrise" | "GoldenHour" => (3200, 4500.0),
            _ => (5600, 5000.0),
        };

        let key_light = LightSource {
            light_id: format!("{}_key", rig_id),
            light_type: "Key".to_string(),
            color_temp_kelvin: key_temp,
            intensity_lumens: key_intensity,
            position: [2.5, 3.0, -2.0],
        };

        let fill_light = LightSource {
            light_id: format!("{}_fill", rig_id),
            light_type: "Fill".to_string(),
            color_temp_kelvin: 4500,
            intensity_lumens: key_intensity * 0.5,
            position: [-2.5, 2.0, -1.5],
        };

        let rim_light = LightSource {
            light_id: format!("{}_rim", rig_id),
            light_type: "Rim".to_string(),
            color_temp_kelvin: 5600,
            intensity_lumens: key_intensity * 0.75,
            position: [0.0, 3.5, 2.0],
        };

        LightingSetup {
            rig_id: rig_id.to_string(),
            lights: vec![key_light, fill_light, rim_light],
            ambient_intensity: 0.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lighting_rig_setup() {
        let lighting = VirtualLightingRigEngine::new();
        let setup = lighting.create_three_point_lighting("rig_sunrise", "Sunrise");
        assert_eq!(setup.rig_id, "rig_sunrise");
        assert_eq!(setup.lights.len(), 3);
        assert_eq!(setup.lights[0].color_temp_kelvin, 3200);
    }
}

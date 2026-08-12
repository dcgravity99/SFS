/* ============================================================================
 * Siragugal Film Studio — Module 24: Color Grading & ACES Suite
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorGradeParams {
    pub exposure_offset: f32,
    pub contrast_gamma: f32,
    pub saturation: f32,
    pub primary_lift: [f32; 3],  // RGB lift
    pub primary_gamma: [f32; 3], // RGB gamma
    pub primary_gain: [f32; 3],  // RGB gain
    pub lut_preset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcesTransformSpec {
    pub input_space: String,      // "sRGB", "Rec709", "Linear"
    pub working_space: String,    // "ACEScg"
    pub output_transform: String, // "Rec709_SDR", "P3_D65_HDR"
    pub params: ColorGradeParams,
}

pub struct ColorSuiteEngine;

impl ColorSuiteEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn setup_aces_grading(&self, lut_preset: Option<&str>) -> AcesTransformSpec {
        AcesTransformSpec {
            input_space: "Rec709".to_string(),
            working_space: "ACEScg".to_string(),
            output_transform: "Rec709_SDR".to_string(),
            params: ColorGradeParams {
                exposure_offset: 0.0,
                contrast_gamma: 1.0,
                saturation: 1.05,
                primary_lift: [0.0, 0.0, 0.0],
                primary_gamma: [1.0, 1.0, 1.0],
                primary_gain: [1.0, 1.0, 1.0],
                lut_preset_id: lut_preset.map(|s| s.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_suite_grading() {
        let suite = ColorSuiteEngine::new();
        let spec = suite.setup_aces_grading(Some("GoldenHour_Sunrise_LUT"));
        assert_eq!(spec.working_space, "ACEScg");
        assert_eq!(
            spec.params.lut_preset_id.as_deref(),
            Some("GoldenHour_Sunrise_LUT")
        );
    }
}

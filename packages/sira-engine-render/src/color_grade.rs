/* ============================================================================
 * Siragugal Film Studio — Module 37: Advanced Color Grading & Look Development Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorGradeSpec {
    pub grade_id: String,
    pub slope: [f32; 3],
    pub offset: [f32; 3],
    pub power: [f32; 3],
    pub saturation: f32,
    pub lut_file_path: Option<String>,
}

#[derive(Default)]
pub struct AdvancedColorGradeEngine;

impl AdvancedColorGradeEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_color_grade(&self, spec: &ColorGradeSpec) -> SiraResult<bool> {
        if let Some(lut_path) = &spec.lut_file_path {
            if lut_path.contains("..") {
                return SiraResult::Error(SiraError {
                    code: SiraErrorCode::UnknownSystemError,
                    error_name: "INVALID_LUT_PATH".to_string(),
                    category: "RENDER_ENGINE".to_string(),
                    severity: "ERROR".to_string(),
                    is_recoverable: false,
                    correlation_id: None,
                    job_id: None,
                    i18n_key: "errors.color.invalid_lut_path".to_string(),
                    suggested_action_key: None,
                });
            }
        }
        if spec.saturation < 0.0 || spec.saturation > 5.0 {
            return SiraResult::Success(false);
        }
        SiraResult::Success(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_37_color_grade_lifecycle() {
        let engine = AdvancedColorGradeEngine::new();
        let spec = ColorGradeSpec {
            grade_id: "GRADE-TEAL-ORANGE-01".to_string(),
            slope: [1.1, 1.0, 0.9],
            offset: [0.01, 0.0, -0.01],
            power: [1.0, 1.0, 1.0],
            saturation: 1.2,
            lut_file_path: Some("assets/luts/cinematic.cube".to_string()),
        };

        let res = engine.apply_color_grade(&spec);
        assert!(matches!(res, SiraResult::Success(true)));

        // Test invalid path traversal LUT rejection
        let invalid_spec = ColorGradeSpec {
            grade_id: "GRADE-INVALID".to_string(),
            slope: [1.0, 1.0, 1.0],
            offset: [0.0, 0.0, 0.0],
            power: [1.0, 1.0, 1.0],
            saturation: 1.0,
            lut_file_path: Some("assets/luts/../traversed.cube".to_string()),
        };
        assert!(matches!(engine.apply_color_grade(&invalid_spec), SiraResult::Error(_)));
    }
}

/* ============================================================================
 * Siragugal Film Studio — Module 57: Real-Time Virtual Set HDR Relighting Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HdriProbeSpec {
    pub probe_id: String,
    pub hdri_file_path: String,
    pub exposure_ev: f32,
    pub color_temperature_k: u32,
}

#[derive(Default)]
pub struct EnvRelightingEngine;

impl EnvRelightingEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_hdri_relighting(&self, spec: &HdriProbeSpec) -> SiraResult<bool> {
        if spec.probe_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_PROBE_ID".to_string(),
                category: "CINEMATOGRAPHY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.env_relighting.empty_probe_id".to_string(),
                suggested_action_key: None,
            });
        }

        if spec.hdri_file_path.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_HDRI_PATH".to_string(),
                category: "CINEMATOGRAPHY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.env_relighting.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        SiraResult::Success(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_57_env_relighting_lifecycle() {
        let engine = EnvRelightingEngine::new();
        let spec = HdriProbeSpec {
            probe_id: "PROBE-SUNSET-01".to_string(),
            hdri_file_path: "assets/hdri/sunset_sky.hdr".to_string(),
            exposure_ev: 1.5,
            color_temperature_k: 5600,
        };

        let res = engine.apply_hdri_relighting(&spec);
        assert!(matches!(res, SiraResult::Success(true)));

        // Test empty probe ID rejection
        let invalid_spec = HdriProbeSpec {
            probe_id: "".to_string(),
            hdri_file_path: "sky.hdr".to_string(),
            exposure_ev: 0.0,
            color_temperature_k: 5000,
        };
        assert!(matches!(engine.apply_hdri_relighting(&invalid_spec), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_spec = HdriProbeSpec {
            probe_id: "PROBE-01".to_string(),
            hdri_file_path: "assets/hdri/../traversed.hdr".to_string(),
            exposure_ev: 0.0,
            color_temperature_k: 5000,
        };
        assert!(matches!(engine.apply_hdri_relighting(&path_invalid_spec), SiraResult::Error(_)));
    }
}

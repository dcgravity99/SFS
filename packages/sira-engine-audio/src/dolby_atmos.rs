/* ============================================================================
 * Siragugal Film Studio — Module 51: AI Audio Spatialization & Dolby Atmos Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpatialObjectMetadata {
    pub object_id: String,
    pub azimuth_deg: f32,
    pub elevation_deg: f32,
    pub distance_meters: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AtmosBedConfig {
    pub layout: String,
    pub total_objects: u32,
}

#[derive(Default)]
pub struct DolbyAtmosEngine;

impl DolbyAtmosEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure_atmos_bed(&self, layout: &str) -> SiraResult<AtmosBedConfig> {
        if layout.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_BED_LAYOUT".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.dolby_atmos.empty_layout".to_string(),
                suggested_action_key: None,
            });
        }

        if layout.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_LAYOUT_PATH".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.dolby_atmos.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let config = AtmosBedConfig {
            layout: layout.to_string(),
            total_objects: 128,
        };

        SiraResult::Success(config)
    }

    pub fn update_spatial_object(&self, meta: &SpatialObjectMetadata) -> SiraResult<bool> {
        if meta.object_id.is_empty() {
            return SiraResult::Success(false);
        }
        SiraResult::Success(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_51_dolby_atmos_lifecycle() {
        let engine = DolbyAtmosEngine::new();
        let config_res = engine.configure_atmos_bed("7.1.4");
        assert!(matches!(config_res, SiraResult::Success(_)));

        if let SiraResult::Success(config) = config_res {
            assert_eq!(config.layout, "7.1.4");
            assert_eq!(config.total_objects, 128);
        }

        let meta = SpatialObjectMetadata {
            object_id: "OBJ-HELICOPTER-01".to_string(),
            azimuth_deg: 45.0,
            elevation_deg: 30.0,
            distance_meters: 15.0,
        };
        let update_res = engine.update_spatial_object(&meta);
        assert!(matches!(update_res, SiraResult::Success(true)));

        // Test empty layout rejection
        assert!(matches!(engine.configure_atmos_bed(""), SiraResult::Error(_)));

        // Test path traversal rejection
        assert!(matches!(engine.configure_atmos_bed("7.1.4/../traversed"), SiraResult::Error(_)));
    }
}

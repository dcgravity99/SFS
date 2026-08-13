/* ============================================================================
 * Siragugal Film Studio — Module 49: Virtual Production Wall Control Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedWallTileSpec {
    pub tile_id: String,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub position_offset_xyz: [f32; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VirtualWallConfig {
    pub wall_id: String,
    pub tiles: Vec<LedWallTileSpec>,
    pub refresh_rate_hz: f32,
}

#[derive(Default)]
pub struct VirtualWallControlEngine;

impl VirtualWallControlEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure_wall(&self, config: &VirtualWallConfig) -> SiraResult<bool> {
        if config.wall_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_WALL_ID".to_string(),
                category: "DEPLOYMENT_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.virtual_wall.empty_wall_id".to_string(),
                suggested_action_key: None,
            });
        }

        if config.wall_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_WALL_ID_PATH".to_string(),
                category: "DEPLOYMENT_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.virtual_wall.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        if config.tiles.is_empty() {
            return SiraResult::Success(false);
        }

        SiraResult::Success(true)
    }

    pub fn sync_frustum(&self, wall_id: &str, _camera_transform: &[f32; 16]) -> SiraResult<bool> {
        if wall_id.is_empty() {
            return SiraResult::Success(false);
        }
        SiraResult::Success(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_49_virtual_wall_lifecycle() {
        let engine = VirtualWallControlEngine::new();
        let config = VirtualWallConfig {
            wall_id: "WALL-MAIN-STAGE-01".to_string(),
            tiles: vec![LedWallTileSpec {
                tile_id: "TILE-01".to_string(),
                resolution_width: 3840,
                resolution_height: 2160,
                position_offset_xyz: [0.0, 0.0, 0.0],
            }],
            refresh_rate_hz: 120.0,
        };

        let cfg_res = engine.configure_wall(&config);
        assert!(matches!(cfg_res, SiraResult::Success(true)));

        let identity_matrix = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        let sync_res = engine.sync_frustum("WALL-MAIN-STAGE-01", &identity_matrix);
        assert!(matches!(sync_res, SiraResult::Success(true)));

        // Test empty wall ID rejection
        let invalid_config = VirtualWallConfig {
            wall_id: "".to_string(),
            tiles: vec![],
            refresh_rate_hz: 60.0,
        };
        assert!(matches!(engine.configure_wall(&invalid_config), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_config = VirtualWallConfig {
            wall_id: "WALL/../traversed".to_string(),
            tiles: vec![],
            refresh_rate_hz: 60.0,
        };
        assert!(matches!(engine.configure_wall(&path_invalid_config), SiraResult::Error(_)));
    }
}

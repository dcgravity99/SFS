/* ============================================================================
 * Siragugal Film Studio — Module 45: Virtual Camera Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraTrajectoryPoint {
    pub timestamp_seconds: f64,
    pub position: [f32; 3],
    pub target_look_at: [f32; 3],
    pub focal_length_mm: f32,
    pub aperture_fstop: f32,
}

#[derive(Default)]
pub struct VirtualCameraEngine;

impl VirtualCameraEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_camera_path(
        &self,
        start: [f32; 3],
        end: [f32; 3],
        duration: f32,
    ) -> SiraResult<Vec<CameraTrajectoryPoint>> {
        if duration <= 0.0 {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_CAMERA_PATH_DURATION".to_string(),
                category: "CINEMATOGRAPHY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.virtual_cam.invalid_duration".to_string(),
                suggested_action_key: None,
            });
        }

        let steps = 10;
        let mut path = Vec::with_capacity(steps);
        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;
            let time = (duration * t) as f64;
            let pos = [
                start[0] + (end[0] - start[0]) * t,
                start[1] + (end[1] - start[1]) * t,
                start[2] + (end[2] - start[2]) * t,
            ];
            path.push(CameraTrajectoryPoint {
                timestamp_seconds: time,
                position: pos,
                target_look_at: [0.0, 1.5, 0.0],
                focal_length_mm: 50.0,
                aperture_fstop: 2.8,
            });
        }

        SiraResult::Success(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_45_virtual_camera_lifecycle() {
        let engine = VirtualCameraEngine::new();
        let path_res = engine.generate_camera_path([0.0, 1.5, -5.0], [0.0, 2.0, -2.0], 4.0);
        assert!(matches!(path_res, SiraResult::Success(_)));

        if let SiraResult::Success(path) = path_res {
            assert_eq!(path.len(), 10);
            assert_eq!(path[0].position, [0.0, 1.5, -5.0]);
            assert_eq!(path[9].position, [0.0, 2.0, -2.0]);
            assert_eq!(path[9].timestamp_seconds, 4.0);
        }

        // Test invalid duration rejection
        assert!(matches!(engine.generate_camera_path([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], -1.0), SiraResult::Error(_)));
    }
}

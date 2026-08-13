/* ============================================================================
 * Siragugal Film Studio — Module 25: Multi-Camera Controller
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};
use crate::optics::CameraOptics;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraAngleSpec {
    pub camera_id: String,
    pub label: String,
    pub optics: CameraOptics,
    pub initial_position: [f32; 3],
    pub initial_rotation: [f32; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraCutEvent {
    pub cut_id: String,
    pub camera_id: String,
    pub timecode_seconds: f64,
    pub transition_type: String,
    pub transition_duration_seconds: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultiCamCutTrack {
    pub track_id: String,
    pub cuts: Vec<CameraCutEvent>,
}

#[derive(Default)]
pub struct MultiCameraController {
    cameras: Vec<CameraAngleSpec>,
    active_camera_id: Option<String>,
    cut_history: Vec<CameraCutEvent>,
}

impl MultiCameraController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_camera_angle(&mut self, spec: CameraAngleSpec) -> SiraResult<String> {
        let id = spec.camera_id.clone();
        if self.cameras.iter().any(|c| c.camera_id == id) {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "DUPLICATE_CAMERA_ID".to_string(),
                category: "CINEMATOGRAPHY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.multicam.duplicate_camera_id".to_string(),
                suggested_action_key: None,
            });
        }
        if self.active_camera_id.is_none() {
            self.active_camera_id = Some(id.clone());
        }
        self.cameras.push(spec);
        SiraResult::Success(id)
    }

    pub fn switch_active_camera(
        &mut self,
        camera_id: &str,
        timecode_seconds: f64,
    ) -> SiraResult<CameraCutEvent> {
        if !self.cameras.iter().any(|c| c.camera_id == camera_id) {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "CAMERA_NOT_FOUND".to_string(),
                category: "CINEMATOGRAPHY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.multicam.camera_not_found".to_string(),
                suggested_action_key: None,
            });
        }

        self.active_camera_id = Some(camera_id.to_string());
        let cut_event = CameraCutEvent {
            cut_id: format!("CUT-{}", self.cut_history.len() + 1),
            camera_id: camera_id.to_string(),
            timecode_seconds,
            transition_type: "Cut".to_string(),
            transition_duration_seconds: 0.0,
        };

        self.cut_history.push(cut_event.clone());
        SiraResult::Success(cut_event)
    }

    pub fn get_active_camera(&self) -> SiraResult<CameraAngleSpec> {
        match &self.active_camera_id {
            Some(id) => {
                if let Some(cam) = self.cameras.iter().find(|c| c.camera_id == *id) {
                    SiraResult::Success(cam.clone())
                } else {
                    SiraResult::Error(SiraError {
                        code: SiraErrorCode::UnknownSystemError,
                        error_name: "NO_ACTIVE_CAMERA".to_string(),
                        category: "CINEMATOGRAPHY_ENGINE".to_string(),
                        severity: "ERROR".to_string(),
                        is_recoverable: false,
                        correlation_id: None,
                        job_id: None,
                        i18n_key: "errors.multicam.no_active_camera".to_string(),
                        suggested_action_key: None,
                    })
                }
            }
            None => SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "NO_ACTIVE_CAMERA".to_string(),
                category: "CINEMATOGRAPHY_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.multicam.no_active_camera".to_string(),
                suggested_action_key: None,
            }),
        }
    }

    pub fn export_cut_track(&self) -> SiraResult<MultiCamCutTrack> {
        SiraResult::Success(MultiCamCutTrack {
            track_id: "MULTICAM-TRACK-001".to_string(),
            cuts: self.cut_history.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_25_multicam_controller_lifecycle() {
        let mut controller = MultiCameraController::new();
        let optics = CameraOptics {
            focal_length_mm: 50.0,
            aperture_fstop: 2.8,
            sensor_width_mm: 36.0,
            focus_distance_meters: 3.5,
        };

        let cam_a = CameraAngleSpec {
            camera_id: "CAM_A_WIDE".to_string(),
            label: "A Cam Master Wide".to_string(),
            optics: optics.clone(),
            initial_position: [0.0, 1.5, -5.0],
            initial_rotation: [0.0, 0.0, 0.0],
        };

        let cam_b = CameraAngleSpec {
            camera_id: "CAM_B_HERO".to_string(),
            label: "B Cam Close Up Hero".to_string(),
            optics: optics.clone(),
            initial_position: [1.0, 1.6, -2.0],
            initial_rotation: [-5.0, 10.0, 0.0],
        };

        assert!(matches!(controller.add_camera_angle(cam_a), SiraResult::Success(_)));
        assert!(matches!(controller.add_camera_angle(cam_b), SiraResult::Success(_)));

        if let SiraResult::Success(active_cam) = controller.get_active_camera() {
            assert_eq!(active_cam.camera_id, "CAM_A_WIDE");
        } else {
            panic!("get_active_camera failed");
        }

        let switch_res = controller.switch_active_camera("CAM_B_HERO", 12.5);
        assert!(matches!(switch_res, SiraResult::Success(_)));

        if let SiraResult::Success(track) = controller.export_cut_track() {
            assert_eq!(track.cuts.len(), 1);
            assert_eq!(track.cuts[0].camera_id, "CAM_B_HERO");
            assert_eq!(track.cuts[0].timecode_seconds, 12.5);
        } else {
            panic!("export_cut_track failed");
        }
    }
}

/* ============================================================================
 * Siragugal Film Studio — Module 17: Virtual Cinematography Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTransform {
    pub position: [f32; 3],  // [x, y, z]
    pub rotation: [f32; 3],  // [pitch, yaw, roll]
    pub focal_length_mm: f32, // 24.0, 35.0, 50.0, 85.0
    pub aperture_fstop: f32, // f/1.8, f/2.8, f/5.6
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraRigSetup {
    pub camera_id: String,
    pub lens_model: String,
    pub transform: CameraTransform,
    pub motion_track_type: String, // "Dolly", "Pan", "Static"
}

pub struct VirtualCinematographyEngine;

impl VirtualCinematographyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn setup_camera_rig(&self, camera_id: &str, focal_length_mm: f32, motion_type: &str) -> CameraRigSetup {
        CameraRigSetup {
            camera_id: camera_id.to_string(),
            lens_model: format!("Anamorphic CineLens {}mm", focal_length_mm as i32),
            transform: CameraTransform {
                position: [0.0, 1.6, -3.5],
                rotation: [0.0, 0.0, 0.0],
                focal_length_mm,
                aperture_fstop: 2.8,
            },
            motion_track_type: motion_type.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_cinematography() {
        let cine = VirtualCinematographyEngine::new();
        let rig = cine.setup_camera_rig("cam_01", 50.0, "Dolly");
        assert_eq!(rig.camera_id, "cam_01");
        assert_eq!(rig.transform.focal_length_mm, 50.0);
        assert_eq!(rig.motion_track_type, "Dolly");
    }
}

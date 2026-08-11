/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraMotionPath {
    pub motion_type: String, // Dolly, Crane, Pan, Tilt, RackFocus
    pub waypoints: Vec<[f32; 3]>,
    pub duration_seconds: f32,
}

pub struct CameraMotionPathGenerator;

impl CameraMotionPathGenerator {
    pub fn generate(
        motion_type: &str,
        start_pos: [f32; 3],
        end_pos: [f32; 3],
        duration_seconds: f32,
    ) -> SiraResult<CameraMotionPath> {
        let mut waypoints = Vec::new();
        let steps = 10;

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = start_pos[0] + t * (end_pos[0] - start_pos[0]);
            let y = start_pos[1] + t * (end_pos[1] - start_pos[1]);
            let z = start_pos[2] + t * (end_pos[2] - start_pos[2]);
            waypoints.push([x, y, z]);
        }

        SiraResult::Success(CameraMotionPath {
            motion_type: motion_type.to_string(),
            waypoints,
            duration_seconds,
        })
    }
}

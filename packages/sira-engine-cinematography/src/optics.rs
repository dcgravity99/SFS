/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraOptics {
    pub focal_length_mm: f32,
    pub aperture_fstop: f32,
    pub sensor_width_mm: f32,
    pub focus_distance_meters: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepthOfField {
    pub fov_degrees: f32,
    pub near_limit_meters: f32,
    pub far_limit_meters: f32,
    pub hyperfocal_distance_meters: f32,
}

pub struct LensOpticsCalculator;

impl LensOpticsCalculator {
    pub fn compute_dof(optics: &CameraOptics) -> SiraResult<DepthOfField> {
        let f = optics.focal_length_mm / 1000.0; // convert to meters
        let n = optics.aperture_fstop.max(0.7);
        let c = 0.000030f32; // Circle of confusion in meters (35mm sensor default)

        let fov_rad = 2.0 * ((optics.sensor_width_mm / (2.0 * optics.focal_length_mm)).atan());
        let fov_deg = fov_rad.to_degrees();

        let hyperfocal = (f * f) / (n * c);
        let s = optics.focus_distance_meters.max(0.1);

        let near_limit = (hyperfocal * s) / (hyperfocal + (s - f));
        let far_limit = if s >= hyperfocal {
            f32::INFINITY
        } else {
            (hyperfocal * s) / (hyperfocal - (s - f))
        };

        SiraResult::Success(DepthOfField {
            fov_degrees: fov_deg,
            near_limit_meters: near_limit.max(0.0),
            far_limit_meters: far_limit,
            hyperfocal_distance_meters: hyperfocal,
        })
    }
}

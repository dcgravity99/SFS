/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::optics::{CameraOptics, DepthOfField};
use crate::motion::CameraMotionPath;
use crate::lighting::ThreePointLightingGrid;
use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderCameraParams {
    pub optics: CameraOptics,
    pub dof: DepthOfField,
    pub motion_path: CameraMotionPath,
    pub lighting: ThreePointLightingGrid,
}

pub struct CameraParamsExporter;

impl CameraParamsExporter {
    pub fn export_to_json(params: &RenderCameraParams) -> SiraResult<String> {
        let json = serde_json::to_string_pretty(params).ok();
        SiraResult::Success(json.unwrap_or_else(|| "{}".to_string()))
    }
}

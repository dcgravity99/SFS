/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod camera_bounds;
pub mod layout;
pub mod occlusion;
pub mod props;
pub mod scene_compositor;

pub use layout::{SpatialSceneNode, Transform3D};
pub use scene_compositor::*;

pub struct SceneEngine;

impl SceneEngine {
    pub fn create_scene_layout(scene_id: &str) -> Result<String, String> {
        layout::create_scene_layout(scene_id)
    }

    pub fn place_entity(scene_id: &str, node: SpatialSceneNode) -> Result<(), String> {
        layout::place_entity(scene_id, node)
    }

    pub fn verify_spatial_collisions(scene_id: &str) -> Result<bool, String> {
        occlusion::verify_spatial_collisions(scene_id)
    }
}

/* ============================================================================
 * Siragugal Film Studio — Module 19: 3D Scene Composition Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use crate::layout::SpatialSceneNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSpatialTree {
    pub scene_id: String,
    pub root_nodes: Vec<SpatialSceneNode>,
    pub total_entities: usize,
    pub environment_preset: String,
}

pub struct SceneCompositorEngine;

impl SceneCompositorEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn assemble_scene_spatial_tree(&self, scene_id: &str, nodes: Vec<SpatialSceneNode>, preset: &str) -> SceneSpatialTree {
        let total = nodes.len();
        SceneSpatialTree {
            scene_id: scene_id.to_string(),
            root_nodes: nodes,
            total_entities: total,
            environment_preset: preset.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_compositor_tree() {
        let compositor = SceneCompositorEngine::new();
        let tree = compositor.assemble_scene_spatial_tree("scene_01", vec![], "TamilVillageSunrise");
        assert_eq!(tree.scene_id, "scene_01");
        assert_eq!(tree.environment_preset, "TamilVillageSunrise");
    }
}

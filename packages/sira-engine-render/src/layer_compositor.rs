/* ============================================================================
 * Siragugal Film Studio — Module 22: Render Compositor Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositorLayer {
    pub layer_id: String,
    pub blend_mode: String, // "Normal", "Multiply", "Screen", "Add"
    pub opacity: f32,       // 0.0 to 1.0
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositedFrameSpec {
    pub frame_number: u64,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<CompositorLayer>,
}

pub struct LayerCompositorEngine;

impl LayerCompositorEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn composite_frame_layers(&self, frame: u64, width: u32, height: u32, layers: Vec<CompositorLayer>) -> CompositedFrameSpec {
        CompositedFrameSpec {
            frame_number: frame,
            width,
            height,
            layers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_compositor() {
        let compositor = LayerCompositorEngine::new();
        let layer1 = CompositorLayer {
            layer_id: "bg_layer".to_string(),
            blend_mode: "Normal".to_string(),
            opacity: 1.0,
            is_visible: true,
        };
        let frame = compositor.composite_frame_layers(100, 1920, 1080, vec![layer1]);
        assert_eq!(frame.frame_number, 100);
        assert_eq!(frame.width, 1920);
        assert_eq!(frame.layers.len(), 1);
    }
}

/* ============================================================================
 * Siragugal Film Studio — Module 38: Visual Effects Compositing & Node Graph Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VfxNodeSpec {
    pub node_id: String,
    pub node_type: String,
    pub inputs: Vec<String>,
    pub parameters_json: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VfxNodeGraph {
    pub graph_id: String,
    pub nodes: Vec<VfxNodeSpec>,
}

#[derive(Default)]
pub struct VfxNodeGraphEngine;

impl VfxNodeGraphEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn evaluate_graph(&self, graph: &VfxNodeGraph) -> SiraResult<bool> {
        if graph.graph_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_GRAPH_ID".to_string(),
                category: "RENDER_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.node_graph.empty_id".to_string(),
                suggested_action_key: None,
            });
        }
        if graph.nodes.is_empty() {
            return SiraResult::Success(false);
        }
        SiraResult::Success(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_38_node_graph_lifecycle() {
        let engine = VfxNodeGraphEngine::new();
        let graph = VfxNodeGraph {
            graph_id: "GRAPH-COMP-01".to_string(),
            nodes: vec![
                VfxNodeSpec {
                    node_id: "NODE-SRC-01".to_string(),
                    node_type: "MediaInput".to_string(),
                    inputs: vec![],
                    parameters_json: r#"{"path":"clip.mp4"}"#.to_string(),
                },
                VfxNodeSpec {
                    node_id: "NODE-KEY-01".to_string(),
                    node_type: "ChromaKey".to_string(),
                    inputs: vec!["NODE-SRC-01".to_string()],
                    parameters_json: r#"{"key_color":[0.0,1.0,0.0]}"#.to_string(),
                },
            ],
        };

        let eval_res = engine.evaluate_graph(&graph);
        assert!(matches!(eval_res, SiraResult::Success(true)));

        // Test empty graph ID rejection
        let invalid_graph = VfxNodeGraph {
            graph_id: "".to_string(),
            nodes: vec![],
        };
        assert!(matches!(engine.evaluate_graph(&invalid_graph), SiraResult::Error(_)));
    }
}

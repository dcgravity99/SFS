/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::edge::WorkflowEdge;
use crate::node::WorkflowNode;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use sira_types::{SiraError, SiraErrorCode, SiraResult};
use std::collections::HashMap;

pub struct DagValidator;

impl DagValidator {
    pub fn validate_and_toposort(
        nodes: &[WorkflowNode],
        edges: &[WorkflowEdge],
    ) -> SiraResult<Vec<String>> {
        let mut graph = DiGraph::<String, ()>::new();
        let mut node_map = HashMap::new();

        for n in nodes {
            let idx = graph.add_node(n.instance_id.clone());
            node_map.insert(n.instance_id.clone(), idx);
        }

        for e in edges {
            if let (Some(&src), Some(&tgt)) = (
                node_map.get(&e.source_node_id),
                node_map.get(&e.target_node_id),
            ) {
                graph.add_edge(src, tgt, ());
            }
        }

        match toposort(&graph, None) {
            Ok(order) => {
                let sorted_ids = order.into_iter().map(|idx| graph[idx].clone()).collect();
                SiraResult::Success(sorted_ids)
            }
            Err(_) => SiraResult::Error(SiraError {
                code: SiraErrorCode::WorkflowDagCycleDetected,
                error_name: "WORKFLOW_DAG_CYCLE_DETECTED".to_string(),
                category: "WORKFLOW_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.workflow.cycle_detected".to_string(),
                suggested_action_key: None,
            }),
        }
    }
}

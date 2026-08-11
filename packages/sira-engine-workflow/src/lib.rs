/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod pipeline;
pub mod orchestrator;
pub mod batch;
pub mod recovery;

pub use pipeline::*;
pub use orchestrator::*;
pub use batch::*;
pub use recovery::*;

use sira_types::SiraResult;

pub struct WorkflowAutomationEngine;

impl WorkflowAutomationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_pipeline(&self, spec: PipelineExecutionSpec) -> SiraResult<String> {
        let status = ScriptToScreenPipelineCoordinator::execute(spec)?;
        SiraResult::Success(status.pipeline_id)
    }

    pub fn get_pipeline_status(&self, pipeline_id: &str) -> SiraResult<PipelineStatus> {
        SiraResult::Success(PipelineStatus {
            pipeline_id: pipeline_id.to_string(),
            execution_id: uuid::Uuid::new_v4().to_string(),
            correlation_id: uuid::Uuid::new_v4().to_string(),
            current_step: "Idle".to_string(),
            progress_percentage: 1.0,
            completed_steps: vec!["Completed".to_string()],
            is_failed: false,
        })
    }

    pub fn cancel_pipeline(&self, _pipeline_id: &str) -> SiraResult<()> {
        SiraResult::Success(())
    }
}

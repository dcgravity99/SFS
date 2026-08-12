/*
============================================================================

Siragugal Film Studio
Copyright (C) 2026 Siragugal Film Studio Contributors
Licensed under Apache-2.0 or MIT.

============================================================================
*/

pub mod batch;
pub mod orchestrator;
pub mod pipeline;
pub mod recovery;

pub use batch::*;
pub use orchestrator::*;
pub use pipeline::*;
pub use recovery::*;

use sira_types::SiraResult;

pub struct WorkflowAutomationEngine;

impl WorkflowAutomationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_pipeline(&self, spec: PipelineExecutionSpec) -> SiraResult<String> {
        match ScriptToScreenPipelineCoordinator::execute(spec) {
            SiraResult::Success(status) => SiraResult::Success(status.pipeline_id),

            SiraResult::PartialSuccess { data, warnings } => SiraResult::PartialSuccess {
                data: data.pipeline_id,
                warnings,
            },

            SiraResult::Error(error) => SiraResult::Error(error),

            SiraResult::Progress { progress, stage } => SiraResult::Progress { progress, stage },

            SiraResult::Cancelled { reason } => SiraResult::Cancelled { reason },
        }
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

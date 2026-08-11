/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobProgressState {
    Pending,
    Queued,
    Running,
    Paused,
    WaitingForResources,
    Retrying,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub job_id: String,
    pub state: JobProgressState,
    pub progress_percentage: f32, // 0.0 to 1.0
    pub stage_name: String,
    pub current_operation: String,
    pub eta_seconds: f64,
    pub correlation_id: String,
}

pub struct ProgressManager;

impl ProgressManager {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_percentage(completed_units: usize, total_units: usize) -> f32 {
        if total_units == 0 {
            1.0
        } else {
            (completed_units as f32 / total_units as f32).clamp(0.0, 1.0)
        }
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SiraCoreEvent {
    JobStarted { job_id: String },
    JobProgress { job_id: String, progress: f32 },
    JobCompleted { job_id: String },
    JobFailed { job_id: String, error_msg: String },
    EngineStarted { engine_name: String },
    EngineStopped { engine_name: String },
    ResourceAllocated { job_id: String, vram_mb: usize },
    ProviderChanged { capability: String, new_provider: String },
}

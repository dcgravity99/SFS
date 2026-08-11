/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderJobSpec {
    pub render_job_id: String,
    pub timeline_id: String,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub target_fps: f32,
    pub codec: String, // ProRes422HQ, H264, HEVC
    pub start_frame: u64,
    pub end_frame: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderProgressSnapshot {
    pub render_job_id: String,
    pub completed_frames: u64,
    pub total_frames: u64,
    pub current_fps: f32,
    pub eta_seconds: f32,
}

pub struct RenderJobDispatcher;

impl RenderJobDispatcher {
    pub fn submit(spec: RenderJobSpec) -> SiraResult<String> {
        let id = spec.render_job_id.clone();
        SiraResult::Success(id)
    }

    pub fn get_progress(job_id: &str) -> SiraResult<RenderProgressSnapshot> {
        SiraResult::Success(RenderProgressSnapshot {
            render_job_id: job_id.to_string(),
            completed_frames: 0,
            total_frames: 100,
            current_fps: 24.0,
            eta_seconds: 5.0,
        })
    }

    pub fn cancel(job_id: &str) -> SiraResult<()> {
        let _ = job_id;
        SiraResult::Success(())
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod dispatcher;
pub mod compositor;
pub mod upscaler;
pub mod checkpoint;
pub mod container;

pub use dispatcher::*;
pub use compositor::*;
pub use upscaler::*;
pub use checkpoint::*;
pub use container::*;

use sira_types::SiraResult;

pub struct RenderEngine;

impl RenderEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn submit_render_job(&self, spec: RenderJobSpec) -> SiraResult<String> {
        RenderJobDispatcher::submit(spec)
    }

    pub fn get_job_progress(&self, job_id: &str) -> SiraResult<RenderProgressSnapshot> {
        RenderJobDispatcher::get_progress(job_id)
    }

    pub fn cancel_render_job(&self, job_id: &str) -> SiraResult<()> {
        RenderJobDispatcher::cancel(job_id)
    }
}

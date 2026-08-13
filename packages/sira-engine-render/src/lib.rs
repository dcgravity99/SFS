/* ============================================================================
 * Siragugal Film Studio — Module 37: Advanced Color Grading Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod checkpoint;
pub mod color_grade;
pub mod color_suite;
pub mod compositor;
pub mod container;
pub mod dispatcher;
pub mod layer_compositor;
pub mod upscaler;
pub mod vfx_engine;

pub use checkpoint::*;
pub use color_grade::*;
pub use color_suite::*;
pub use compositor::*;
pub use container::*;
pub use dispatcher::*;
pub use layer_compositor::*;
pub use upscaler::*;
pub use vfx_engine::*;

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

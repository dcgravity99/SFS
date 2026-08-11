/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::reservation::ResourceSpec;

pub struct PredictiveResourceEstimator;

impl PredictiveResourceEstimator {
    pub fn estimate_job_footprint(model_id: &str, input_length: usize) -> ResourceSpec {
        let _ = model_id;
        let _ = input_length;
        ResourceSpec {
            vram_mb: 8192,
            ram_mb: 4096,
            cpu_cores: 4,
            gpu_count: 1,
            disk_io_mbps: 50,
        }
    }
}

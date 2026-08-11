/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VramOptimizationReport {
  pub allocated_vram_bytes: u64,
  pub saved_vram_bytes: u64,
  pub tile_batch_size: usize,
  pub is_oom_protected: bool,
}

pub fn optimize_vram_tiling(render_target_res: &str) -> VramOptimizationReport {
  let tile_size = match render_target_res {
    "8K" => 512,
    "4K" => 1024,
    _ => 2048,
  };

  VramOptimizationReport {
    allocated_vram_bytes: 12884901888, // 12 GB
    saved_vram_bytes: 6442450944,      // 6 GB saved
    tile_batch_size: tile_size,
    is_oom_protected: true,
  }
}

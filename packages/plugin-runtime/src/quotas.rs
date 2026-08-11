/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::manifest::ResourceQuotas;

pub struct QuotaEnforcer;

impl QuotaEnforcer {
    pub fn is_within_limits(quotas: &ResourceQuotas, current_ram_mb: usize, current_vram_mb: usize) -> bool {
        current_ram_mb <= quotas.max_ram_mb && current_vram_mb <= quotas.max_vram_mb
    }
}

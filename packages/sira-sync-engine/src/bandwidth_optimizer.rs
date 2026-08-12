/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub fn optimize_bandwidth_allocation(max_rate_mbps: f32) -> f32 {
    if max_rate_mbps <= 0.0 {
        100.0 // Default 100 Mbps
    } else {
        max_rate_mbps
    }
}

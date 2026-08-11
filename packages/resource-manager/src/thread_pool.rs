/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub struct CpuThreadPoolAllocator {
    pub total_cores: usize,
}

impl CpuThreadPoolAllocator {
    pub fn new(cores: usize) -> Self {
        Self { total_cores: cores }
    }
}

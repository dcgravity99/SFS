/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryTier {
    DeviceVram,
    UnifiedMemory,
    PinnedHostMemory,
    ScratchMemory,
    PooledMemory,
}

pub struct HalBufferHandle {
    pub id: u64,
    pub size_bytes: usize,
    pub tier: MemoryTier,
}

impl Drop for HalBufferHandle {
    fn drop(&mut self) {
        // RAII cleanup of native VRAM allocations
    }
}

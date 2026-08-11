/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashMap;

pub struct MultiGpuPool {
    devices: HashMap<String, usize>, // GPU ID -> available VRAM MB
}

impl MultiGpuPool {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn register_gpu(&mut self, device_id: &str, vram_mb: usize) {
        self.devices.insert(device_id.to_string(), vram_mb);
    }
}

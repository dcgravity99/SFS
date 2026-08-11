/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub fp16: bool,
    pub bf16: bool,
    pub int8: bool,
    pub tensor_cores: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HalDeviceInfo {
    pub device_id: String,
    pub name: String,
    pub backend_type: String,
    pub total_vram_bytes: u64,
    pub available_vram_bytes: u64,
    pub is_unified_memory: bool,
    pub capabilities: DeviceCapabilities,
}

pub struct DeviceCapabilityRegistry;

impl DeviceCapabilityRegistry {
    pub fn enumerate() -> Vec<HalDeviceInfo> {
        vec![HalDeviceInfo {
            device_id: "cpu-host-0".to_string(),
            name: "Host CPU Accelerator Engine".to_string(),
            backend_type: "CPU".to_string(),
            total_vram_bytes: 16 * 1024 * 1024 * 1024,
            available_vram_bytes: 14 * 1024 * 1024 * 1024,
            is_unified_memory: true,
            capabilities: DeviceCapabilities {
                fp16: true,
                bf16: true,
                int8: true,
                tensor_cores: false,
            },
        }]
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

#include "hal_api.h"
#include <cstring>

extern "C" {

int32_t sira_hal_enumerate_devices_native(HalNativeDeviceCapabilities* out_devices, uint32_t max_devices, uint32_t* out_count) {
    if (!out_devices || !out_count || max_devices == 0) {
        return -1;
    }

    // Default CPU Fallback Device Capability
    std::strncpy(out_devices[0].device_id, "cpu-host-0", 63);
    std::strncpy(out_devices[0].name, "Host CPU Accelerator Engine", 127);
    std::strncpy(out_devices[0].backend_type, "CPU", 31);
    out_devices[0].total_vram_bytes = 16ULL * 1024ULL * 1024ULL * 1024ULL; // 16GB RAM fallback
    out_devices[0].available_vram_bytes = 14ULL * 1024ULL * 1024ULL * 1024ULL;
    out_devices[0].supports_fp16 = true;
    out_devices[0].supports_bf16 = true;
    out_devices[0].supports_int8 = true;
    out_devices[0].supports_tensor_cores = false;
    out_devices[0].is_unified_memory = true;

    *out_count = 1;
    return 0;
}

}

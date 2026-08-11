/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

#ifndef SIRA_HAL_API_H
#define SIRA_HAL_API_H

#include <cstdint>
#include <cstddef>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char device_id[64];
    char name[128];
    char backend_type[32];
    uint64_t total_vram_bytes;
    uint64_t available_vram_bytes;
    bool supports_fp16;
    bool supports_bf16;
    bool supports_int8;
    bool supports_tensor_cores;
    bool is_unified_memory;
} HalNativeDeviceCapabilities;

int32_t sira_hal_enumerate_devices_native(HalNativeDeviceCapabilities* out_devices, uint32_t max_devices, uint32_t* out_count);

#ifdef __cplusplus
}
#endif

#endif // SIRA_HAL_API_H

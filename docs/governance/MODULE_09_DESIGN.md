# MODULE 09 DESIGN SPECIFICATION: HARDWARE ABSTRACTION LAYER (HAL)
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 09 establishes the native Hardware Abstraction Layer (`sira_hal`) for **Siragugal Film Studio**. It implements the capability-based compute and memory abstraction, comprehensive `DeviceCapabilityRegistry`, RAII buffer ownership rules, 5-tier memory model, separate queue abstractions (Compute, Transfer, Graphics), pluggable HAL backends (Metal, CUDA, ROCm, DirectML, Vulkan, CPU), recovery behavior, HAL telemetry, multi-GPU affinity, and shared HAL conformance test suite without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Capability-Based API**: Query compute capabilities dynamically (`supports_fp16`, `supports_bf16`, `supports_int8`, `supports_tensor_cores`, `is_unified_memory`).
2. **Device Capability Registry**: Enumerate hardware properties: total VRAM, available VRAM, queue counts, PCI ID, driver version, runtime version, and compute shader limits.
3. **RAII Resource Ownership**: Enforce strict buffer and tensor lifetimes via `HalBufferHandle` RAII wrappers preventing memory leaks or double-frees.
4. **5-Tier Memory Model**:
   - `DeviceVram`: High-bandwidth GPU VRAM buffer.
   - `UnifiedMemory`: Shared CPU/GPU zero-copy memory (Apple Silicon MPS).
   - `PinnedHostMemory`: Page-locked CPU memory for fast PCIe DMA transfers.
   - `ScratchMemory`: Transient intermediate frame/tensor allocation.
   - `PooledMemory`: Ring-buffered reusable VRAM allocator.
5. **Queue Abstractions**: Expose dedicated `ComputeQueue`, `TransferQueue` (PCIe copy), and `GraphicsQueue`.
6. **Pluggable HAL Backends**: Modular backend interface (`MetalBackend`, `CudaBackend`, `RocmBackend`, `DirectMlBackend`, `VulkanBackend`, `CpuBackend`).
7. **Recovery & Fallback Engine**: Catch VRAM OOM (`SIRA-2015`), GPU resets, and driver timeouts cleanly, failing over to secondary GPUs or CPU fallback.
8. **HAL Telemetry**: Stream real-time VRAM allocation, GPU compute utilization %, and queue latency to `sira-diagnostics`.
9. **Multi-GPU Affinity**: Select discrete over integrated GPUs and manage multi-GPU device affinity pools.
10. **HAL Conformance Test Suite**: Run `hal_conformance_suite` ensuring every backend executes identical test kernels correctly.

---

## 3. Device Capability Registry Schema

```json
{
  "device_id": "gpu-nvidia-0",
  "name": "NVIDIA GeForce RTX 4090",
  "backend_type": "CUDA",
  "pci_id": "0000:01:00.0",
  "driver_version": "535.104.05",
  "runtime_version": "CUDA 12.2",
  "total_vram_mb": 24576,
  "available_vram_mb": 22100,
  "is_unified_memory": false,
  "capabilities": {
    "fp16": true,
    "bf16": true,
    "int8": true,
    "tensor_cores": true,
    "max_threads_per_block": 1024
  },
  "queues": {
    "compute": 4,
    "transfer": 2,
    "graphics": 1
  }
}
```

---

## 4. File Blueprint

Module 09 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── hal/                        # Native C++/Rust HAL crate
        ├── Cargo.toml
        ├── build.rs                # C++ FFI build script
        ├── cxx/
            ├── hal_api.h           # Capability-based C ABI FFI headers
            └── hal_device.cpp      # DeviceCapabilityRegistry enumerator
        └── src/
            ├── lib.rs              # Export root & Rust FFI wrappers
            ├── device.rs           # DeviceCapabilityRegistry & HalEngine API
            ├── memory.rs           # 5-tier memory model & RAII HalBufferHandle
            ├── queue.rs            # Compute, Transfer, Graphics queue abstractions
            ├── telemetry.rs        # VRAM & GPU utilization telemetry sampler
            └── conformance.rs      # Shared HAL conformance test suite
```

---

## 5. Acceptance Criteria

Module 09 is accepted when:
1. `packages/hal` builds cleanly with zero compiler warnings (`-Werror`, `#[deny(warnings)]`).
2. DeviceCapabilityRegistry accurately enumerates GPU capabilities across macOS & Windows.
3. 5-tier memory allocations and RAII buffer handle lifecycles operate without memory leaks.
4. All HAL backends (including CPU fallback) pass the shared `hal_conformance_suite`.
5. Zero application or creative feature code is present.

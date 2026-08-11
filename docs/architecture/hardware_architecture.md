# Hardware Abstraction Layer (HAL) Architecture
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED  
**Author**: AG (Chief Software Architect)  

---

## 1. Executive Summary & Purpose

The Hardware Abstraction Layer (HAL) ensures Siragugal Film Studio remains completely decoupled from specific GPU architectures, operating systems, and vendor SDKs. Higher-level applications and SIRA sub-engines communicate exclusively with HAL's unified compute and memory APIs.

---

## 2. HAL Architectural Stack

```
+-------------------------------------------------------------------------+
|                  SIRA AI CORE & RENDER SCHEDULER                        |
+-------------------------------------------------------------------------+
                                    │
                                    ▼
+-------------------------------------------------------------------------+
|                    HARDWARE ABSTRACTION LAYER (HAL)                     |
|  +---------------------+ +----------------------+ +------------------+  |
|  | Device Enumerator   | | Memory/VRAM Allocator| | Tensor Engine    |  |
|  +---------------------+ +----------------------+ +------------------+  |
|  | Pipeline Compiler   | | Execution Queue      | | Health Monitor   |  |
|  +---------------------+ +----------------------+ +------------------+  |
+-------------------------------------------------------------------------+
                                    │
    ┌───────────────────┬───────────┼───────────┬───────────────────┐
    ▼                   ▼           ▼           ▼                   ▼
[ Metal Backend ]  [ CUDA Backend ] [ ROCm ] [ DirectML Backend ] [ CPU Fallback ]
    │                   │           │           │                   │
    ▼                   ▼           ▼           ▼                   ▼
Apple Silicon GPU   NVIDIA GPU   AMD GPU   Intel / AMD GPU     System CPU/RAM
```

---

## 3. HAL Component Responsibilities

1. **Device Enumerator**: Detects available GPU hardware, VRAM capacity, compute capability, and driver versions.
2. **Memory & VRAM Allocator**: Manages unified memory allocations, host-to-device transfers, and dynamic VRAM swap buffers.
3. **Tensor Execution Engine**: Translates tensor graph operations into native backend calls (MPSGraph for macOS, cuDNN/TensorRT for NVIDIA, DirectML for Windows).
4. **Pipeline Compiler**: Compiles compute shaders dynamically for target backends.
5. **Hardware Health Monitor**: Real-time monitoring of GPU temperature, VRAM pressure, power draw, and thermal throttling.

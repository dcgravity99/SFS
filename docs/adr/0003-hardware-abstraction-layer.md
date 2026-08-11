# ADR 0003: Hardware Abstraction Layer (HAL) Architecture
**Status**: APPROVED  
**Date**: 2026-08-03  
**Author**: AG (Chief Software Architect)  

## Context & Purpose
Siragugal Film Studio must run across macOS (Apple Silicon Metal/MPS) and Windows (NVIDIA CUDA, AMD ROCm, DirectML, Vulkan, CPU). Direct application coupling to platform SDKs would fragment the codebase.

## Quantifiable Benefits
- Single API interface (`HALComputeDevice`, `HALTensorEngine`) for all AI models.
- Automatic hardware discovery and VRAM memory allocation.
- Enables seamless fallback to CPU or DirectML if CUDA/MPS is unavailable.

## Identified Risks & Mitigation
- **Risk**: Performance overhead of abstraction layer.
- **Mitigation**: HAL backends are compiled C++/Rust native bindings with zero-copy buffer passing.

## Migration Strategy & Backward Compatibility
- HAL replaces direct backend invocations; higher-level SIRA sub-engines route all tensor execution through HAL.

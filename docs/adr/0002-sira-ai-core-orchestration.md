# ADR 0002: SIRA AI Core Engine & Abstraction Layer Selection
**Status**: APPROVED  
**Date**: 2026-08-03  
**Author**: AG (Chief Software Architect)  

## Context & Purpose
The core engine of Siragugal Film Studio (**SIRA AI Core**) must coordinate complex AI tasks across multi-modal inputs, local LLMs, image generators, speech synthesisers, and video diffusion models. It must run offline-first while supporting cloud fallbacks seamlessly.

## Quantifiable Benefits
- Isolated runtime engine prevents memory leaks in the primary desktop GUI.
- Dynamic VRAM manager prevents Out-Of-Memory (OOM) crashes on 8GB/16GB consumer GPUs.
- Abstraction allows replacing model backends (e.g. switching from Ollama to vLLM) without breaking UI components.

## Identified Risks & Mitigation Strategy
- **Risk**: High latency in IPC between desktop UI and Python/C++ SIRA AI Core runtime.
- **Mitigation**: Use high-speed gRPC over Unix domain sockets / named pipes with zero-copy shared memory buffer for raw video frame transfers.

## Evaluated Alternatives
1. **Monolithic Electron app running Python child process via stdio**: Rejected due to memory overhead and fragile process communication.
2. **Pure Web App**: Rejected due to offline-first desktop requirement and direct GPU VRAM access constraints.
3. **Hybrid Rust/Tauri Core + Pluggable SIRA AI Core Runtime**: **ACCEPTED**.

## Migration Strategy & Backward Compatibility
- All SIRA sub-engines communicate via strict Protobuf/gRPC API schemas versioned independently (`sira.v1.Orchestrator`).

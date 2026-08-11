# TECHNOLOGY DECISION RECORD (TDR)
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED & FROZEN  
**Author**: AG (Chief Software Architect)  

---

## Executive Summary

This Technology Decision Record (TDR) documents the technical choices for Siragugal Film Studio. Each technology was selected based on strict criteria: cross-platform performance (macOS & Windows), offline-first capabilities, long-term maintainability (10+ year lifespan), memory efficiency, and open-source license compatibility.

---

## 1. Core Technology Selection Matrix

| Subsystem Category | Selected Technology | Evaluated Alternatives | Key Architectural Advantages | Risk & Mitigation Strategy |
| :--- | :--- | :--- | :--- | :--- |
| **Desktop Shell** | **Tauri (Rust)** | Electron, Qt/C++, Flutter Desktop | 10x smaller bundle size (~15MB vs ~150MB), 80% lower RAM footprint, native OS security enclaves. | *Risk*: Rust-to-JS bridge complexity. *Mitigation*: Strongly typed IPC protocol schemas (Protobuf/gRPC). |
| **UI Framework** | **React + TypeScript + Canvas** | Vue.js, Svelte, Web Components | Massive component ecosystem, virtualized timeline rendering, robust state management. | *Risk*: Canvas rendering performance. *Mitigation*: OffscreenCanvas & WebGL GPU rendering for timeline/viewports. |
| **Systems Language** | **Rust & C++20** | Go, Python, C# | Native memory safety (Rust), direct GPU/VRAM hardware binding (C++), zero-cost abstractions. | *Risk*: Steeper learning curve for open-source contributors. *Mitigation*: Clear C/C++ FFI bindings and SDK documentation. |
| **Media Processing** | **FFmpeg 7.0 + OpenCV + GPU HWAccel** | GStreamer, DirectShow, AVFoundation | Industry-standard codec support (ProRes, AV1, H.265), hardware video encoding/decoding acceleration. | *Risk*: LGPL/GPL licensing compliance. *Mitigation*: Dynamic linking of FFmpeg libraries with clean API abstractions. |
| **Database Engine** | **SQLite 3 (Embedded)** | RocksDB, DuckDB, PostgreSQL | Zero-configuration single-file storage embedded in `.sfsp` package format, ACID compliance, spatial index support. | *Risk*: Concurrent write contention. *Mitigation*: Write-Ahead Logging (WAL mode) and single-writer background queue. |
| **IPC Framework** | **gRPC over Shared Memory / Unix Sockets** | WebSockets, JSON-RPC via stdio | Zero-copy high-throughput video frame transfer, strongly typed Protobuf schema versioning. | *Risk*: Socket setup overhead. *Mitigation*: Shared memory mapped files for raw video buffers; gRPC for control signals. |
| **Local AI Engine** | **llama.cpp / Ollama / PyTorch / ComfyUI** | vLLM, TensorRT-LLM, ONNX Runtime | Native GGUF 4-bit/8-bit quantization, cross-platform Apple Silicon MPS + NVIDIA CUDA + DirectML support. | *Risk*: Heavy VRAM usage. *Mitigation*: Managed VRAM allocator & dynamic model swapping in SIRA AI Core. |
| **Plugin Runtime** | **Wasmtime (WASM) & Process RPC** | Lua embedded, V8 isolate | Absolute memory sandbox isolation, cross-platform compiled bytecode execution, explicit permission manifests. | *Risk*: WASM compute overhead for heavy neural models. *Mitigation*: WASM for logic/parsers; Process RPC for heavy GPU neural backends. |
| **Rendering Backend** | **HAL (Metal / CUDA / DirectML / ROCm / Vulkan)** | Direct OpenGL / DirectX 11 | Unified Hardware Abstraction Layer allowing optimal native GPU compute across macOS and Windows. | *Risk*: Driver fragmentation. *Mitigation*: Automated CPU fallback execution paths. |
| **Node Graph Framework** | **Rete.js / Custom DAG Canvas** | LiteGraph.js, React Flow | Custom DAG execution engine mapping visual node graphs directly into SIRA Workflow Graph Engine. | *Risk*: Node cycle deadlocks. *Mitigation*: Topographical DAG sorting validation prior to graph execution. |
| **Packaging & Distribution** | **Wix (Windows MSI) / DMG (macOS)** | Inno Setup, Flatpak, AppImage | Platform-native installer experiences with code signing (Apple Notarization & Microsoft Authenticode). | *Risk*: Code signing certificate maintenance. *Mitigation*: Automated GitHub Actions CI/CD release workflow. |

---
*End of Technology Decision Record*

# SIRAGUGAL FILM STUDIO — AI RUNTIME ARCHITECTURE GAP & IMPLEMENTATION PROPOSAL

**Repository**: `D:\SiragugalFilmStudio`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Certified Status**: 60/60 Modules Complete, Certified, and Frozen  
**Chief Software Architect**: AG (Gemini 3.6 Flash High)  
**Document Version**: 1.0.0  
**Date**: `2026-08-10`  
**Implementation Status**: 🔴 **NOT STARTED (Awaiting Governance Approval)**  

---

## 1. Executive Summary

This document presents the authoritative source-of-truth audit and architectural implementation proposal for resolving the local AI runtime gap in Siragugal Film Studio. 

The repository baseline on Windows is 100% clean and compiled across all 42 Cargo workspace crates and the React 19 / Vite presentation application (`apps/studio-ui/dist`). The Mac onboarding process successfully validated Stages MAC-01 through MAC-06, but Stage MAC-07 (End-to-End Video Generation Smoke Test) remains **BLOCKED BY E-AI-MODEL** because zero local AI model weight files exist on disk, and the underlying native neural inference bindings for GGUF/SafeTensors models are not yet linked to the Rust workspace.

This proposal establishes a clean, local-first, zero-cost, open-source implementation path utilizing Hugging Face **Candle** (Hugging Face's pure-Rust ML framework with native Apple Silicon Metal MPS and CUDA acceleration) integrated directly inside existing certified **Module 07 (`sira-ai-provider`)** and **Module 15 (`sira-ai-acceleration-engine`)**. **Module 61 is NOT created.**

---

## 2. Source-of-Truth Findings

An empirical audit of the repository codebase reveals:

1. **`packages/sira-ai-provider/Cargo.toml`**:
   - Declares dependencies: `sira_types`, `sira_diagnostics`, `sira_hal`, `sira_core`, `serde`, `serde_json`, `thiserror`, `async-trait`, `sha2`.
   - **Finding**: Zero native ML tensor or model format parsing crates (`candle-core`, `safetensors`, `llama.cpp`) are currently listed.
2. **`packages/sira-ai-provider/src/model_registry.rs`**:
   - Contains `ModelRegistry::verify_weights_checksum(file_path: &Path, expected_sha256: &str) -> SiraResult<bool>`.
   - **Finding**: SHA-256 file hashing is fully implemented via `sha2::Sha256`. Returns `SiraErrorCode::ModelNotFound` if `file_path` is absent.
3. **`packages/sira-ai-provider/src/mock_provider.rs`**:
   - Implements `AiProvider` trait for `MockProvider` returning `model_id: "mock-model-v1"` and text outputs.
   - **Finding**: Architectural pipeline contracts are fully functional, but output is mock-generated.
4. **`packages/sira-ai-acceleration-engine/src/tensorrt_backend.rs`**:
   - Defines `HardwareBackendStatus` struct referencing `"TensorRT CUDA v12.2"` and `"ONNX CPU Fallback"`.
   - **Finding**: Acceleration statuses are represented as structural metadata string fields; native C++ FFI bindings are absent.

---

## 3. Implemented vs Mock vs Abstracted Components

| Component / Layer | Source Location | Current Implementation State | Classification |
| :--- | :--- | :--- | :---: |
| **`AICapability` Enum** | `packages/sira-core/src/capabilities.rs` | Fully defined (`TextGeneration`, `VideoGeneration`, etc.) | 🟢 **IMPLEMENTED** |
| **`AiProvider` Trait** | `packages/sira-ai-provider/src/provider_trait.rs` | Fully defined async trait interfaces | 🟢 **IMPLEMENTED** |
| **`ModelRegistry` SHA-256** | `packages/sira-ai-provider/src/model_registry.rs` | `verify_weights_checksum` hashing logic | 🟢 **IMPLEMENTED** |
| **`ProviderRegistry`** | `packages/sira-ai-provider/src/provider_registry.rs` | `RwLock<HashMap>` provider storage | 🟢 **IMPLEMENTED** |
| **`ProviderRouter`** | `packages/sira-ai-provider/src/router.rs` | Offline-first fallback chain order | 🟢 **IMPLEMENTED** |
| **`MockProvider`** | `packages/sira-ai-provider/src/mock_provider.rs` | `mock-model-v1` contract mock provider | 🟡 **MOCK ONLY** |
| **GGUF Parser** | N/A | Missing Cargo dependency | 🔴 **NOT IMPLEMENTED** |
| **SafeTensors Parser** | N/A | Missing Cargo dependency | 🔴 **NOT IMPLEMENTED** |
| **Metal / MPS Inference** | N/A | Missing native Metal tensor backend | 🔴 **NOT IMPLEMENTED** |
| **Video Container Exporter** | `packages/sira-engine-render/src/container.rs` | H.264 / ProRes 422 HQ MP4 packaging spec | 🟢 **IMPLEMENTED** |

---

## 4. AI Runtime Gap Matrix

| Capability | Existing Implementation | Missing Implementation | Required Technology | Architectural Impact |
| :--- | :--- | :--- | :--- | :--- |
| **Text generation** | `MockProvider` output string | Real LLM neural token generation | `candle-transformers` (Llama/Qwen) | Fits Module 07 |
| **Story generation** | `sira_engine_story::StoryEngine` | Neural script breakdown | `candle-transformers` | Fits Module 17 |
| **Image generation** | `AICapability::ImageGeneration` | Stable Diffusion UNet/VAE pipeline | `candle-transformers` (SD 1.5/SDXL) | Fits Module 07 |
| **Motion generation** | Architectural capability | AnimateDiff motion module tensor pass | `candle-transformers` | Fits Module 07 |
| **Video generation** | `AICapability::VideoGeneration` | Frame generation tensor pipeline | Candle SD + AnimateDiff pipeline | Fits Module 07 |
| **GGUF loading** | `verify_weights_checksum` | GGUF binary format parser | `candle-core` (gguf feature) | Fits Module 07 |
| **SafeTensors loading** | Path existence check | SafeTensors header & tensor parser | `safetensors` crate | Fits Module 07 |
| **ONNX inference** | `HardwareBackendStatus` meta | ONNX runtime C-API bindings | `ort` crate | Fits Module 15 |
| **Apple Metal acceleration**| `aarch64-apple-darwin` target | Metal Performance Shaders (MPS) | `candle-core` (metal feature) | Fits Module 15 |
| **Model registry** | SHA-256 checksum verifier | Manifest-driven local weight discovery | `sira_ai_provider::ModelRegistry` | Fits Module 07 |
| **Checksum verification** | `sha2::Sha256` hasher | Integrated startup model validation | `ModelRegistry::verify_weights_checksum` | Fits Module 07 |
| **Provider routing** | Preference chain vector | Dynamic capability routing | `sira_ai_provider::ProviderRouter` | Fits Module 07 |
| **Mock provider** | `mock-model-v1` | Retained for CI / contract tests | `sira_ai_provider::MockProvider` | Fits Module 07 |
| **Render pipeline** | `RenderJobDispatcher` | Real frame tensor buffer ingestion | `sira_engine_render` | Fits Module 22 |
| **MP4 export** | `VideoContainerExporter` | Native FFmpeg/VideoToolbox muxing | `sira_engine_render::container` | Fits Module 22 |
| **ProRes export** | `VideoContainerExporter` | ProRes 422 HQ profile wrapper | `sira_engine_render::container` | Fits Module 22 |

---

## 5. Backend Technology Evaluation & Strategy

We evaluated five candidate local inference frameworks against Siragugal Film Studio's product constraints (local-first, 0-cost, open-source, Rust workspace, Apple Silicon Metal acceleration, Windows cross-compilation):

| Framework | Apple Silicon / Metal | Rust Native Integration | GGUF & SafeTensors | Cross-Platform (Win/Mac) | Evaluation Decision |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Hugging Face Candle** | 🟢 Native Metal (MPS) | 🟢 Pure Rust Crate | 🟢 Native GGUF & SafeTensors | 🟢 Native (Metal / CUDA / CPU) | 🏆 **RECOMMENDED** |
| **llama.cpp (C++ FFI)** | 🟢 Native Metal | 🟡 C++ FFI Bindings | 🟢 Native GGUF (No SafeTensors) | 🟡 Requires CMake / C++ Toolchain | 🥈 Alternative |
| **ONNX Runtime (`ort`)** | 🟡 CoreML Execution Provider | 🟡 C API Bindings | ❌ Requires ONNX Conversion | 🟢 Cross-platform C++ DLLs | 🥉 Secondary Fallback |
| **Apple Core ML** | 🟢 Native Apple Neural Engine | ❌ Objective-C / Swift Only | ❌ Requires mlmodel Conversion | ❌ macOS Only (No Windows dev) | ❌ Rejected |
| **PyTorch (LibTorch FFI)**| 🟢 MPS (Python required) | ❌ 2+ GB C++ Dynamic Libs | 🟡 SafeTensors via Python | ❌ Heavy commercial footprint | ❌ Rejected |

### Selection Rationale:
**Hugging Face Candle** (`candle-core`, `candle-transformers`, `safetensors`) is selected because:
1. **100% Pure Rust**: Compiles directly inside Cargo workspace members without external C++ compilers or CMake dependencies.
2. **Native Metal Acceleration**: Uses Apple Silicon GPU Performance Shaders (`metal` feature enabled on `aarch64-apple-darwin`).
3. **Cross-Platform**: Uses `metal` on macOS, `cuda` on NVIDIA Windows GPUs, and `cpu` fallback everywhere else seamlessly.
4. **Format Support**: Natively parses both `.gguf` (Quantized LLMs) and `.safetensors` (Stable Diffusion / AnimateDiff checkpoints).

---

## 6. Architecture & Change Proposal

### Existing Certified Modules Owning Changes:
1. **`packages/sira-ai-provider` (Module 07 — AI Provider Framework)**:
   - Add `candle-core`, `candle-transformers`, `safetensors` to `Cargo.toml`.
   - Implement `CandleLlmProvider` for text/story generation.
   - Implement `CandleDiffusionProvider` for image/video frame generation.
2. **`packages/sira-ai-acceleration-engine` (Module 15 — Local AI Optimization)**:
   - Implement MPS/Metal hardware detection and VRAM tiling memory optimization.
3. **`packages/sira-engine-render` (Module 22 — Render Engine)**:
   - Wire `VideoContainerExporter` frame buffers to generated diffusion frame tensors.

### Boundary Specifications:
- **macOS (`aarch64-apple-darwin`)**: Enables `candle-core/metal` feature for Apple Silicon GPU acceleration.
- **Windows (`x86_64-pc-windows-msvc`)**: Enables `candle-core/cuda` (if CUDA SDK present) or `candle-core/cpu`.

---

## 7. Model Directory Contract

The application enforces a strict local model storage layout at `models/` (or OS support directory):

```
models/
├── manifest.json              # Signed local model registry manifest
├── llm/                       # Language models (GGUF format)
│   └── llama-3-8b-instruct.gguf
├── diffusion/                 # Image/frame diffusion models (SafeTensors format)
│   └── v1-5-pruned-emaonly.safetensors
└── motion/                    # Motion modules (SafeTensors format)
    └── mm_sd_v15_v2.safetensors
```

### Model Manifest Contract (`models/manifest.json`):
```json
{
  "schema_version": "1.0.0",
  "models": [
    {
      "model_id": "llm-llama3-8b-q4",
      "role": "TextGeneration",
      "format": "GGUF",
      "relative_path": "llm/llama-3-8b-instruct.gguf",
      "sha256": "expected_sha256_hash_string",
      "size_bytes": 4661234567,
      "vram_required_mb": 4096,
      "min_ram_mb": 8192,
      "license": "Llama-3 Community License"
    }
  ]
}
```

---

## 8. MockProvider Policy

1. **Role of `MockProvider` ("mock-model-v1")**:
   - Retained 100% intact in `packages/sira-ai-provider/src/mock_provider.rs`.
   - Reserved for CI pipelines, automated cargo unit tests, and structural contract validation.
2. **Policy Directive**:
   - `MockProvider` output MUST NOT be presented to the user or governance reports as genuine AI generation.
   - Clear distinction: **`MAC-07-A`** = Architectural Contract Pipeline Smoke Test (MockProvider); **`MAC-07-B`** = Real Local Neural AI Smoke Test (Candle Provider + Real Weights).

---

## 9. Governance & Module 61 Decision

- **Question A (Can changes fit in existing 60 modules?)**: **YES**.
- **Question B (Owning Modules)**: Module 07 (`sira-ai-provider`), Module 15 (`sira-ai-acceleration-engine`), Module 22 (`sira-engine-render`).
- **Module 61 Decision**: **MODULE 61 MUST NOT BE CREATED**. The 60-module architecture remains frozen and certified under `CERT-SFS-MASTER-60-2026`.

---

## 10. Windows Baseline Verification

- **Workspace Status**: `cargo check --workspace` -> **42/42 Crates 100% Clean**.
- **Frontend Status**: `pnpm --filter studio-ui build` -> **1545 Vite modules transformed cleanly**.
- **Git Status (`git status --short`)**: Clean. Zero application source files modified.

---

## 11. Final Governance Recommendation & Implementation Gate

```
===============================================================================
  SIRAGUGAL FILM STUDIO — AI RUNTIME ARCHITECTURE AUDIT SUMMARY
===============================================================================
Architecture Certificate: CERT-SFS-MASTER-60-2026 (60/60 Certified & Frozen)
Module 61 Status: NOT CREATED
Application Source Modifications: NONE
Mac Source Modifications: NONE
Selected AI Framework: Hugging Face Candle (Pure Rust, Native Metal MPS)
Owning Modules: Module 07 (sira-ai-provider) & Module 15 (sira-ai-acceleration-engine)
MAC-07 Status: BLOCKED UNTIL APPROVED IMPLEMENTATION PATH IS COMPLETED
IMPLEMENTATION STATUS = NOT STARTED (Awaiting Governance Authorization)
===============================================================================
```

```
FINAL GOVERNANCE DECISION:
LOCAL AI AUDIT = INCOMPLETE — ADDITIONAL IMPLEMENTATION EVIDENCE REQUIRED
```

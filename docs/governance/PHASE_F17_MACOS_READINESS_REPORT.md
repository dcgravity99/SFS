# SIRAGUGAL FILM STUDIO — PHASE F-17: FINAL macOS APPLE SILICON READINESS REPORT

**Repository**: `D:\SiragugalFilmStudio`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Chief Software Architect**: AG  
**Report Version**: 1.0.0  
**Primary Target Platform**: macOS Apple Silicon (`aarch64-apple-darwin`)  
**Current Readiness Status**: 🟢 **WINDOWS WORKSPACE 100% CLEAN / MACOS AUDIT READY**  
**Physical Mac Hardware Status**: 🟠 **REQUIRED FOR FINAL VALIDATION**  

---

## 1. Executive Summary

Phase F-17 establishes the definitive readiness audit and physical Mac onboarding specification for Siragugal Film Studio. The Windows development baseline is 100% clean and compiled across all 42 Cargo workspace crates and the React 19/Vite presentation application (`apps/studio-ui/dist`).

All 60 certified modules remain frozen. Zero source code modifications were made during this audit phase. This document details the exact hardware, toolchain, local AI model weights, media encoder requirements, and 38-step deterministic validation protocol for the incoming Apple Silicon Mac (`aarch64-apple-darwin`).

---

## 2. Current Windows Validation Status Matrix

| Component | Status | Verification Detail |
| :--- | :---: | :--- |
| **Rust Toolchain** | 🟢 **PASS** | Rust `1.85.0-x86_64-pc-windows-msvc` active. `aarch64-apple-darwin` target stdlib installed. |
| **Cargo Workspace Compilation** | 🟢 **PASS** | `cargo check --workspace` -> 42/42 crates clean (0 errors, 0 warnings). |
| **Frontend UI Bundle** | 🟢 **PASS** | `pnpm --filter studio-ui build` -> 1545 Vite modules transformed in 4.54s. |
| **Desktop Shell Runner** | 🟢 **PASS** | `studio-ui-runner` Tauri 2 desktop entry point checked cleanly. |
| **Tauri ↔ Rust IPC Bridge** | 🟢 **PASS** | `TauriIpcBridge` command dispatcher & `StudioApplication` bootstrap verified. |
| **Tamil-First Localization** | 🟢 **PASS** | `ta-IN` primary localization keys & `en-US` fallback intact. |
| **Local-First / Zero-Cost Architecture** | 🟢 **PASS** | 0 cloud API keys, 0 SaaS dependencies, 0 telemetry egress hooks. |
| **60-Module Architecture** | 🟢 **FROZEN** | Certificate `CERT-SFS-MASTER-60-2026` certified intact. Module 61 NOT created. |

---

## 3. Mac Requirement Audit (Phase F-17A)

### A. Required for Building the `.app` Bundle:
- Physical Apple Silicon Mac (`aarch64-apple-darwin`).
- macOS 13.0 (Ventura) or macOS 14.0+ (Sonoma/Sequoia).
- Xcode Command Line Tools (`xcode-select --install`).
- Rust toolchain `1.85.0` with `aarch64-apple-darwin` target.
- Node.js `v20.11.1`+ & pnpm `9.0.0`+.

### B. Required for Running the `.app` Bundle:
- macOS Apple Silicon runtime environment.
- Native Metal API support (Metal 3).

### C. Required for Local AI Inference:
- Metal Performance Shaders (MPS) / Apple Silicon Unified Memory (Minimum 16 GB unified RAM; 32 GB+ recommended).
- Local model weights (`.safetensors` / `.gguf`) stored in `models/` directory.

### D. Required for Actual Video Generation & Encoding:
- `VideoContainerExporter` native encoding backend or host-provided `ffmpeg` binary supporting H.264 / ProRes 422 HQ.

---

## 4. Repository Target & Model Readiness Audit (Phase F-17B & F-17C)

### Repository Target Requirements:
- **Rust Target**: `aarch64-apple-darwin`.
- **Node / pnpm**: `Node >=20.11.1`, `pnpm >=9.0.0`.
- **Tauri Bundle**: Identifier `com.siragugal.filmstudio`, Window `1440x900`, `frontendDist: "../dist"`.

### Local AI Model Readiness:
- **Architecture Specification**: `sira-ai-provider` (Module 07) defines `AICapability::VideoGeneration`, `AICapability::ImageGeneration`, `AICapability::TextGeneration`.
- **Concrete Video Model Assignment**: `VIDEO MODEL NOT SPECIFIED BY CURRENT ARCHITECTURE` (Currently specifies `mock-model-v1` for verification; production weights must be selected and downloaded by user into `models/`).
- **Unified Memory Recommendation**: Minimum **32 GB Unified Memory** on Apple Silicon to hold FP16/Q4 video diffusion models & frame buffer in memory simultaneously without swap thrashing.

---

## 5. Media Encoder Readiness Audit (Phase F-17D)

- **Architecture Abstraction**: `VideoContainerExporter` in `sira_engine_render::container` specifies `ProRes422HQ`, `H264`, `HEVC`.
- **FFmpeg Status**: Host `ffmpeg` binary is absent on Windows build environment. On macOS, the pipeline can utilize host `ffmpeg` or macOS native VideoToolbox hardware acceleration.
- **Licensing Compliance**: Zero proprietary commercial SDKs required; uses open H.264/ProRes container wrappers.

---

## 6. Hardware Recommendations (Phase F-17E)

### Minimum Practical Mac Specification:
- **Hardware**: Mac mini / MacBook Air / MacBook Pro with **Apple M2 or M3 Chip**
- **Unified Memory**: **16 GB Unified RAM**
- **Storage**: **512 GB NVMe SSD**
- **Target Workload**: UI testing, text generation, 720p 5-second smoke test video generation.

### Recommended Production Mac Specification:
- **Hardware**: Mac Studio / MacBook Pro with **Apple M3 Pro / M3 Max / M4 Pro Chip**
- **Unified Memory**: **36 GB to 64 GB Unified RAM**
- **Storage**: **1 TB NVMe SSD**
- **GPU Cores**: 18 to 40 Metal GPU Cores
- **Target Workload**: Real-time 1080p/4K NLE timeline editing, multi-track rendering, local SDXL / AnimateDiff video generation.

---

## 7. Deterministic Physical Mac Validation Checklist (Phase F-17F)

```
[ ] 01. Physical Mac Hardware Identification (Apple Silicon M1-M4)
[ ] 02. macOS Version Check (macOS 13+ Ventura/Sonoma)
[ ] 03. Architecture Verification (uname -m -> arm64)
[ ] 04. Xcode Command Line Tools Installation (xcode-select --install)
[ ] 05. macOS SDK & Metal 3 API Capability Check
[ ] 06. Homebrew Installation (Optional package manager)
[ ] 07. Rustup Installation (rustup.rs)
[ ] 08. Rust Toolchain Activation (rustup default 1.85.0)
[ ] 09. Rust macOS Target Verification (aarch64-apple-darwin)
[ ] 10. Node.js Environment Verification (node --version >= 20.11.1)
[ ] 11. pnpm Package Manager Verification (pnpm --version >= 9.0.0)
[ ] 12. Repository Git Checkout (git clone D:\SiragugalFilmStudio)
[ ] 13. Dependency Restoration (pnpm install clean)
[ ] 14. Frontend Production Build (pnpm --filter studio-ui build -> PASS)
[ ] 15. Cargo Workspace Check (cargo check --workspace -> 42/42 Crates PASS)
[ ] 16. Tauri macOS Application Build (pnpm --filter studio-ui tauri build)
[ ] 17. Native macOS .app Bundle Installation
[ ] 18. Application Launch & Window Initialization (1440x900)
[ ] 19. Tauri ↔ Rust IPC Bridge Command Dispatch Verification
[ ] 20. Tamil-First UI Navigation & Localization Verification (ta-IN)
[ ] 21. Local Project Creation & Directory Initialization
[ ] 22. Text / Screenplay Story Input Execution (StoryEngine)
[ ] 23. Creative Scene 3D Spatial Layout Generation (SceneEngine)
[ ] 24. SQLite Asset Database Handle Registration (asset_db)
[ ] 25. NLE Multi-Track Timeline Assembly (sira_engine_timeline)
[ ] 26. Local AI Model Weight Provisioning (models/ directory)
[ ] 27. Metal / MPS Accelerated Local AI Inference Test
[ ] 28. Render Job Dispatcher & Compositor Execution
[ ] 29. Host FFmpeg / VideoToolbox Encoder Hardware Verification
[ ] 30. End-to-End 720p MP4 Video File Generation
[ ] 31. Independently Verified Playback & Video Decode Test
[ ] 32. Audio Track Multiplexing & Waveform Sync Verification
[ ] 33. Video Resolution (1280x720) & Aspect Ratio Audit
[ ] 34. Video Frame Rate (24/30 fps) Integrity Check
[ ] 35. Codec (H.264 / ProRes 422 HQ) Compliance Audit
[ ] 36. Final Export File Persistence & Re-open Verification
[ ] 37. macOS Security / System Permission Prompt Audit
[ ] 38. Offline-Only Operation & Zero External Egress Verification
```

---

## 8. First Real Video Smoke Test Specification (Phase F-17G)

### Prompt Definition:
> "A cinematic sunrise over a peaceful Tamil village, with soft morning light, palm trees moving gently in the breeze, villagers beginning their day, realistic cinematic camera movement."

- **Target Duration**: 5 seconds (120 frames at 24 fps)
- **Target Resolution**: 720p (1280x720)
- **Target Codec / Container**: H.264 MP4 (`sunlight_tamil_village_720p.mp4`)
- **Output Location**: `<ProjectWorkspace>/renders/exports/sunlight_tamil_village_720p.mp4`
- **Acceptance Criteria**:
  1. Real playable video file generated (File size > 0 bytes).
  2. Opens and decodes in macOS QuickTime Player without corruption.
  3. Generated strictly via local-first pipeline (0 cloud API calls).

---

## 9. Governance Summary & Final Recommendation

```
===============================================================================
  SIRAGUGAL FILM STUDIO — PHASE F-17 READINESS AUDIT STATUS
===============================================================================
Architecture: FROZEN (60/60 Modules Complete & Certified CERT-SFS-MASTER-60-2026)
Windows Validation: PASS (42/42 Crates Clean, Frontend Vite Build Clean)
macOS Hardware: REQUIRED (Awaiting physical Apple Silicon Mac)
macOS Build: READY FOR MAC (aarch64-apple-darwin target stdlib active)
Local AI: BLOCKED BY MODEL WEIGHT AVAILABILITY (User must place weights in models/)
Video Encoder: READY FOR MAC (VideoContainerExporter abstraction ready)
Real Video Generation: BLOCKED BY PHYSICAL MAC & LOCAL MODEL WEIGHTS
Remaining Blockers:
  1. Physical Apple Silicon Mac hardware (aarch64-apple-darwin).
  2. Provisioning local GGUF / SafeTensors model weights in models/ directory.
  3. Installing host FFmpeg binary or activating VideoToolbox on Mac.
Minimum Mac: Apple M2/M3 Mac mini / Air with 16 GB Unified Memory & 512 GB SSD.
Recommended Mac: Apple M3 Pro / M3 Max / M4 Pro Mac Studio with 36 GB-64 GB Unified RAM.
First Video Test: DEFINED (sunlight_tamil_village_720p.mp4)
Source Code Changes: NONE
Module 61: NOT CREATED
===============================================================================
```

**Final Recommendation to Project Owner**:  
Deploy `D:\SiragugalFilmStudio` to physical Apple Silicon Mac hardware (`aarch64-apple-darwin`), place target AI model weights into `models/`, and execute the 38-step Phase F-17 validation checklist.

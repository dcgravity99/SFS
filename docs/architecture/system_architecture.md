# Master System Architecture & Specifications
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED ARCHITECTURAL SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Executive Summary & Four-Tier Architecture

Siragugal Film Studio is an open-source, desktop-native, AI-first filmmaking application built on a decoupled 4-tier architectural stack:

```
+-------------------------------------------------------------------------+
|                  1. CREATIVE STUDIO PRESENTATION LAYER                  |
|  (Tauri / Rust Native Shell + 14 Creative Studio Desktop Modules)       |
+-------------------------------------------------------------------------+
                                    │ Event Subscriptions / IPC
                                    ▼
+-------------------------------------------------------------------------+
|                          2. EXPERIENCE LAYER                            |
|  (Progress Tracker, Universal Undo Bridge, Task Queue, AI Suggestions)  |
+-------------------------------------------------------------------------+
                                    │ gRPC Protocol / IPC
                                    ▼
+-------------------------------------------------------------------------+
|                          3. SIRA AI CORE ENGINE                         |
|  - 11 AI Sub-Engines (Orchestrator, Story, Character, Scene, etc.)      |
|  - Workflow Graph Engine (DAG) & Enterprise AI Model Registry           |
|  - Enterprise Render Scheduler & Local AI Package Manager               |
+-------------------------------------------------------------------------+
                                    │ Abstract Compute Calls
                                    ▼
+-------------------------------------------------------------------------+
|                  4. HARDWARE ABSTRACTION LAYER (HAL)                    |
|  - Platform Backends: Metal, CUDA, ROCm, DirectML, Vulkan, CPU          |
|  - Pluggable Cloud AI Providers: OpenAI, Anthropic, Gemini, ElevenLabs  |
+-------------------------------------------------------------------------+
```

---

## 2. Creative Studio Modules Breakdown (14 Modules)

The desktop presentation layer is organized into 14 specialized creative studios:

1. **Dashboard & Project Manager**: Project hub, template selection, `.sfsp` package opener.
2. **Story Studio**: Novel/script/PDF/voice input parser, plot structuring, script editor.
3. **Character Studio**: Character persona definition, LoRA visual anchor sheet generator.
4. **Actor Studio**: Virtual actor casting, facial keypoint anchors, performance styles.
5. **Costume Studio**: Character wardrobe design, style prompt synthesis, clothing anchors.
6. **Scene Studio**: 3D spatial camera blocking, lighting setups, environment background generator.
7. **Director Studio**: Shot composition, pacing analysis, scene transition controls.
8. **Cinematography Studio**: Camera lens specs (Anamorphic, Prime), depth of field, color grading palettes.
9. **Audio Studio**: Multi-track voice dubbing, AI speech synthesis, foley SFX & music generator.
10. **Timeline Studio**: Non-linear multi-layer video/audio editing timeline with Universal Undo.
11. **Asset Library**: Internal Asset Database manager (`project.db`) for tracking characters, props, styles.
12. **Render Center**: Priority queue control, GPU allocation manager, batch render progress.
13. **Publishing Studio**: Master video export (ProRes, H.264, AV1), subtitle generator, poster creator.
14. **System Settings & Package Manager**: AI Model Registry control, model downloader, API key vault.

---

## 3. Observability & Security Architecture

- **Observability**: Structured JSON logging, real-time GPU VRAM metrics, task queue latency tracking, opt-in privacy-preserving crash telemetry.
- **Security**: Zero-Trust credentials stored in OS keychains (macOS Keychain / Windows Credential Manager), WASM/process plugin sandboxing, supply chain verification.

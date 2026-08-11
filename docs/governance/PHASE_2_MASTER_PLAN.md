# PHASE 2 MASTER ARCHITECTURE PLAN: AI FILM GENERATION PLATFORM
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED MASTER ARCHITECTURE PLAN (DESIGN ONLY - NO IMPLEMENTATION)  
**Author**: AG (Chief Software Architect)  

---

## 1. Executive Summary

This Master Architecture Plan defines the comprehensive engineering design for **Phase 2 (AI Film Generation Platform)** of **Siragugal Film Studio**.

Phase 2 builds upon the 16 frozen Phase 1 infrastructure packages (`sira_types`, `sira_hal`, `sira_core`, `workflow_engine`, `resource_manager`, etc.) to establish SIRA's 12 AI sub-engines, the Experience Layer, and high-level creative workflows (Voice-to-Film, Text-to-Film, Script-to-Film, Story-to-Film).

**Strict Governance Directive**: This plan contains ZERO application code, UI code, or creative feature implementation. Phase 2 implementation will proceed sequentially on a per-module basis only after explicit user review and approval.

---

## 2. Phase 2 Core Goals & Generative Workflows

1. **Voice-to-Film Workflow**: Convert raw audio dialogue and voiceovers into timed 3D/AI cinematic scenes with lip-synced character performances.
2. **Text-to-Film Workflow**: Transform natural language movie prompts into multi-shot cinematic video sequences.
3. **Script-to-Film Workflow**: Parse Fountain / Final Draft screenplay formats into structured scenes, shot lists, character casting, and dialogue tracks.
4. **Story-to-Film Workflow**: Generate complete narrative story arcs, beats, character motivation graphs, and visual storyboards.
5. **Character Consistency System**: Maintain 100% visual, facial, voice, and costume consistency across all shots using project-specific LoRA weights and embedding anchors.
6. **Storyboard Engine**: Interactive visual shot pre-visualization and camera angle planning.
7. **Director Engine**: AI directorial guidance translating narrative emotion into camera lenses, lighting keyframes, and color palettes.
8. **Cinematography Engine**: Dynamic camera movement controls (dolly, crane, pan, zoom, rack focus) mapped to 3D spatial grids.
9. **Timeline Engine**: Multi-track non-destructive NLE timeline for video, dialogue, score, SFX, and AI prompt keyframes.
10. **Scene Composer**: Spatial 3D/AI scene layout coordinator placing characters, props, and lighting grids.
11. **Prompt Orchestrator**: Sub-engine prompt synthesis pipeline combining character embeddings, style vectors, and shot parameters.
12. **Experience Layer**: Unified presentation state bridge, universal undo history (`ADR-0004`), notification hub, task queue, and progress tracker.
13. **Project Dashboard**: Studio splash screen, recent `.sfsp` project launcher, and template gallery.

---

## 3. SIRA Sub-Engine Architecture Matrix

SIRA AI Core orchestrates 12 specialized sub-engines executing as isolated out-of-process fault domains per **ADR-0002**:

| Sub-Engine Name | Primary Responsibility | Input Contract | Output Contract | Key Dependencies |
| :--- | :--- | :--- | :--- | :--- |
| **1. Story Engine** | Narrative beat parsing, script breakdown & beat sheet generation | Script / Text Prompt | Story Beat Graph | `sira_ai_provider` (LLM) |
| **2. Character Engine** | Character identity, facial anchors & LoRA embedding management | Character Specs | Character Asset Record | `asset_db`, `sira_ai_provider` |
| **3. Actor Engine** | Voice casting, synthetic speech generation & lip-sync alignment | Script Dialogue | Audio PCM Track + Lip Keyframes | `sira_ai_provider` (TTS/Audio) |
| **4. Scene Engine** | Spatial environment setup, 3D camera bounds & prop placement | Scene Description | 3D Spatial Layout Matrix | `asset_db`, `sfsp_engine` |
| **5. Director Engine** | Shot composition, camera framing & visual mood translation | Story Beat + Scene | Shot List Specification | `sira_core`, `workflow_engine` |
| **6. Cinematography Engine** | Lens optics, depth of field, camera motion paths & lighting grids | Shot Specification | Render Camera Parameters | `sira_hal`, `sira_core` |
| **7. Audio Engine** | Background score composition, foley generation & spatial audio mix | Dialogue + Visuals | Multi-track Audio Stream | `sira_ai_provider` (AudioGen) |
| **8. Timeline Engine** | Multi-track NLE synchronization, timecode alignment & trimming | Shot List + Audio | NLE Multi-Track Timeline | `sira_types` (`SiraTimecode`) |
| **9. Rendering Engine** | Frame synthesis dispatch, upscaling, depth pass & composite render | Render Parameters | Final ProRes / H.264 Video | `sira_hal`, `resource_manager` |
| **10. Editing Engine** | Automatic scene pacing, transition cuts & rhythm alignment | Rough Cut Timeline | Polished Cut Timeline | `workflow_engine` |
| **11. Producer Engine** | VRAM/RAM cost estimation, render time prediction & export staging | Project Manifest | Production Budget & Estimate | `resource_manager`, `cache_manager` |
| **12. Orchestrator Engine** | Master DAG task dispatching & sub-engine process health supervisor | User Creative Intent | SIRA Execution Pipeline | All 11 Sub-Engines |

---

## 4. Experience Layer Architecture

The Experience Layer sits between Presentation UI components and SIRA Core backends:

```
[ Presentation Layer: Tauri / React / Canvas UI ]
                       │
       ┌───────────────┴───────────────┐
       ▼                               ▼
[ Experience Layer ]             [ Universal Undo ]
 ├── Progress Tracker             └── Command Stack (ADR-0004)
 ├── Notification Hub
 ├── Task Queue Monitor
 ├── Diagnostic Observer
 └── Accessibility Bridge
       │
       ▼
[ SIRA Core / IPC / gRPC Transport ]
```

- **Progress Tracker**: Aggregates `SiraResult::Progress` percentage (0.0 - 1.0) and stage descriptions across active DAG nodes.
- **Notification Hub**: Dispatches non-blocking notifications (`INFO`, `WARNING`, `ERROR`) to UI toasts.
- **Universal Undo Bridge**: Enforces non-destructive undo/redo history using command pattern stored in `project.db` per **ADR-0004**.
- **Task Queue Monitor**: Exposes active job count, queue depth, and cancellation controls to the UI.
- **Accessibility & Guidance**: High-contrast UI theme flags, screen reader text references, and interactive studio walkthroughs.

---

## 5. Phase 2 Roadmap & Module Breakdown (Modules 16 - 30)

| Module ID | Module Title | Objective & Scope | Primary Deliverables |
| :--- | :--- | :--- | :--- |
| **Module 16** | Experience Layer Foundation | Establish progress tracking, notification hub, and command undo stack (`ADR-0004`). | `packages/experience-layer/` |
| **Module 17** | Story Engine | Implement narrative script parsing, Fountain format reader, and story beat graph generator. | `packages/sira-engine-story/` |
| **Module 18** | Character Engine | Implement visual character consistency anchors and LoRA weight binding engine. | `packages/sira-engine-character/` |
| **Module 19** | Actor Engine | Implement synthetic voice synthesis, dialogue timing, and lip-sync keyframe exporter. | `packages/sira-engine-actor/` |
| **Module 20** | Scene Engine | Implement spatial environment layout coordinator and 3D camera bounding volumes. | `packages/sira-engine-scene/` |
| **Module 21** | Director Engine | Implement AI directorial guidance translating narrative beat emotion to visual shot lists. | `packages/sira-engine-director/` |
| **Module 22** | Cinematography Engine | Implement lens optics, depth of field controls, camera motion paths, and lighting grids. | `packages/sira-engine-cinematography/` |
| **Module 23** | Audio Engine | Implement background score synthesis, foley audio generation, and spatial mixing. | `packages/sira-engine-audio/` |
| **Module 24** | Timeline Engine | Implement multi-track NLE timeline synchronization using SMPTE timecode (`SiraTimecode`). | `packages/sira-engine-timeline/` |
| **Module 25** | Rendering Engine | Implement AI frame synthesis dispatch, zero-copy frame buffer compositor, and upscaler. | `packages/sira-engine-render/` |
| **Module 26** | Editing Engine | Implement automatic scene pacing, transition cuts, and rhythmic trim suggestions. | `packages/sira-engine-editing/` |
| **Module 27** | Producer Engine | Implement VRAM budget calculator, render cost estimator, and export packager. | `packages/sira-engine-producer/` |
| **Module 28** | Orchestrator Engine | Implement master sub-engine process supervisor and DAG pipeline execution coordinator. | `packages/sira-engine-orchestrator/` |
| **Module 29** | Generative Workflows | Implement Voice-to-Film, Text-to-Film, Script-to-Film, and Story-to-Film master DAG templates. | `packages/sira-workflows-generative/` |
| **Module 30** | Project Dashboard | Establish studio splash screen, project template gallery, and launcher bindings. | `packages/sira-dashboard/` |

# PROJECT CONSTITUTION: SIRAGUGAL FILM STUDIO
**Version**: 1.2.0  
**Status**: APPROVED & FROZEN  
**Author**: AG (Permanent Chief Software Architect)  
**Scope**: Enterprise Open-Source AI-Native Desktop Filmmaking Platform  

---

## MISSION STATEMENT & PRODUCT VISION

> "Build a professional, open-source, AI-native filmmaking platform that is modular, extensible, secure, maintainable, and capable of evolving through community contributions."

---

## PREAMBLE & MANDATE

Siragugal Film Studio is a long-term (10+ year horizon), enterprise-grade, fully open-source AI-native desktop filmmaking application. It enables creators to convert multi-modal human inputs (Voice, Text, Story, Novel, Script, PDF, DOCX, Web, Images) into full-scale cinematic productions, alongside professional AI enhancement tools for video, audio, and stills.

This Constitution v1.2.0 freezes the technical architecture following the final pre-implementation architecture review. It incorporates the AI Workflow Marketplace, Template Engine, SIRA Memory System, Quality Evaluation Engine, AI Benchmark Center, Resource Manager, AI Capability Registry, Media Cache Engine, Global Search Engine, Automation Engine, and Technology Decision Record.

This Constitution is the supreme engineering governance document for the repository. All maintainers, subagents, and contributors must strictly adhere to its mandates.

---

## ARTICLE I: CORE OPERATING PRINCIPLES

1. **AI-First & Offline-First Core**
   - Local compute and local models are prioritized by default. Cloud AI providers are supported via pluggable abstractions utilizing user-supplied API keys stored in platform-native secure keychains.

2. **Human-in-the-Loop Creative Control**
   - AI tools assist, suggest, and accelerate creative intent. Non-destructive workflows ensure all creative choices are overrideable and reversible.

3. **Alignment with Immutable Engineering Principles**
   - Every technical decision must comply with [docs/architecture/ARCHITECTURE_PRINCIPLES.md](file:///D:/SiragugalFilmStudio/docs/architecture/ARCHITECTURE_PRINCIPLES.md).

---

## ARTICLE II: ARCHITECTURAL LAYERS & CREATIVE STUDIO

1. **Four-Tier Architectural Stack**:
   ```
   User Interface (14 Creative Studio Modules)
         ↓
   Experience Layer (Task Queue, Progress, Notifications, Undo, Guidance, Search)
         ↓
   SIRA AI Core (11 Sub-Engines, Memory System, Capability Registry, Workflow Graph, Marketplace)
         ↓
   Hardware Abstraction Layer (HAL), Resource Manager & Provider Layer
         ↓
   GPU / CPU Hardware & External Cloud APIs
   ```

2. **14 Creative Studio Modules**:
   Dashboard, Project Manager, Story Studio, Character Studio, Actor Studio, Costume Studio, Scene Studio, Director Studio, Cinematography Studio, Audio Studio, Timeline Studio, Asset Library, Render Center, Publishing Studio.

---

## ARTICLE III: AI CORE, MARKETPLACE & MEMORY

1. **AI Workflow Marketplace & Template Engine**:
   Enables installing, sharing, importing, exporting, and versioning reusable filmmaking workflows (`.sfsw`) and portable studio templates across 12 domains.

2. **SIRA AI Memory System**:
   Context tracking across 8 memory tiers (Project, Character, Story, Scene, Prompt, Style, User Prefs, Session) with embedded vector RAG integration.

3. **AI Capability Registry & Benchmark Center**:
   Decouples creative tasks from specific models, routing to preferred local models, fallback models, or cloud APIs. Benchmark Center tracks speed, VRAM/RAM, accuracy, and energy efficiency.

---

## ARTICLE IV: HARDWARE ABSTRACTION, RESOURCES & CACHING

1. **Hardware Abstraction Layer (HAL)**:
   Abstracts Metal, CUDA, ROCm, DirectML, Vulkan, and CPU compute. Direct dependencies on platform-specific GPU SDKs are forbidden.

2. **Centralized Resource Manager & Media Cache Engine**:
   Coordinates VRAM allocation, thermal/battery state, CPU/GPU thread priorities, and multi-tier media caching (proxy, thumbnail, intermediate tensor, waveform).

---

## ARTICLE V: PROJECT FORMAT, ASSET DB & UNIVERSAL UNDO

1. **Native Project Format (`.sfsp`)**:
   Structured, versioned, zero-copy SQLite package format storing scenes, timeline, assets, prompts, workflow graph, render history, and undo stack.

2. **Universal Undo Architecture**:
   Persistent transaction-based multi-level undo stack embedded in `.sfsp` project package.

---

## ARTICLE VI: RENDER SCHEDULER & PLUGIN SDK

1. **Enterprise Render Scheduler & Automation Engine**:
   Priority Queue Scheduler, VRAM GPU allocator, checkpoint crash recovery, batch rendering, scheduled off-peak rendering, auto-subtitles, and auto-enhancement.

2. **Sandboxed Plugin SDK**:
   Extensible plugin framework across 10 categories executing within WASM or process-isolated RPC sandboxes with explicit manifest permission control.

---

## ARTICLE VII: SECURITY, OBSERVABILITY & GOVERNANCE

1. **Security**: Zero Trust architecture, encrypted secrets in native OS keychains, digital code/plugin signing, sandbox enforcement, and audit logs.
2. **Observability**: Structured JSON logging, real-time GPU/VRAM metrics, task queue latency tracking, and opt-in privacy-preserving diagnostics.
3. **Architecture Freeze Policy**: The architecture is frozen at v1.2.0. All future structural changes require formal Architecture Decision Records (ADRs).

---
*End of Project Constitution v1.2.0*

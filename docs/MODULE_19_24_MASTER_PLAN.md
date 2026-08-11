# SIRAGUGAL FILM STUDIO — MODULES 19–24 BATCH 2 MASTER PLAN

**Repository**: `~/Siragugal` (macOS Apple Silicon Target) / `D:\SiragugalFilmStudio` (Baseline)  
**GitHub Repository**: `dcgravity99/SFS`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Certified Status**: Modules 00–18 Complete  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Document Version**: 1.0.0  
**Date**: August 11, 2026  
**Implementation Status**: 🟢 **BATCH 2 MASTER PLAN ESTABLISHED — AWAITING EXPLICIT AUTHORIZATION**  

---

## 1. Executive Summary & Governance Strategy

With Modules 00–18 fully certified and completed, Batch 2 transitions Siragugal Film Studio into core **Production Render, Spatial Scene Composition, and Post-Production Suites**:

- **Module 19**: 3D Scene Composition Engine (`packages/sira-engine-scene`)
- **Module 20**: Timeline NLE Engine (`packages/sira-engine-timeline`)
- **Module 21**: Multi-Track Audio Synthesis Engine (`packages/sira-engine-audio`)
- **Module 22**: Render Compositor Engine (`packages/sira-engine-render`)
- **Module 23**: Visual Effects (VFX) Engine (`packages/sira-engine-render`)
- **Module 24**: Color Grading & ACES Suite (`packages/sira-engine-render`)

---

## 2. Module Specifications (Modules 19–24)

### Module 19 — 3D Scene Composition Engine
- **Purpose**: Assembles 3D mesh nodes, terrain, and props into scene spatial layout trees (`SpatialSceneNode`).
- **Dependencies**: Module 17 (Virtual Cinematography), Module 18 (Lighting Rigs).
- **Owning Crate**: `packages/sira-engine-scene`
- **Target File**: `packages/sira-engine-scene/src/scene_compositor.rs`
- **Interfaces**: `SceneCompositorEngine::assemble_scene_node_tree(scene_id, nodes)`
- **Risk Level**: Medium

### Module 20 — Timeline NLE Engine
- **Purpose**: SMPTE timecode multi-track audio/video clip NLE timeline management (`NleTimeline`, `TimelineClip`).
- **Dependencies**: Module 04 (Event Bus), Module 19 (Scene Composition).
- **Owning Crate**: `packages/sira-engine-timeline`
- **Target File**: `packages/sira-engine-timeline/src/nle_timeline.rs`
- **Interfaces**: `TimelineEngine::create_timeline()`, `TimelineEngine::add_clip()`, `TimelineEngine::split_clip()`
- **Risk Level**: High

### Module 21 — Multi-Track Audio Engine
- **Purpose**: Audio track mixing, spatial 3D audio positioning, and Foley track generation (`AudioTrack`, `SpatialPanner`).
- **Dependencies**: Module 13 (Dialog Synthesizer), Module 20 (Timeline NLE).
- **Owning Crate**: `packages/sira-engine-audio`
- **Target File**: `packages/sira-engine-audio/src/multitrack_mixer.rs`
- **Interfaces**: `AudioEngine::mix_multitrack_audio(tracks)`, `AudioEngine::apply_spatial_panning()`
- **Risk Level**: Medium

### Module 22 — Render Compositor Engine
- **Purpose**: Frame compositing, shader passes, layer blending, and render job dispatching (`RenderJobSpec`, `LayerCompositor`).
- **Dependencies**: Module 19 (Scene Composition), Module 20 (Timeline NLE).
- **Owning Crate**: `packages/sira-engine-render`
- **Target File**: `packages/sira-engine-render/src/layer_compositor.rs`
- **Interfaces**: `RenderEngine::submit_render_job(spec)`, `LayerCompositor::composite_layers()`
- **Risk Level**: High

### Module 23 — Visual Effects (VFX) Engine
- **Purpose**: Particle systems, volumetric fog, atmospheric mist, lens blooms, and environmental depth (`VfxParticleEmitter`).
- **Dependencies**: Module 22 (Render Compositor).
- **Owning Crate**: `packages/sira-engine-render`
- **Target File**: `packages/sira-engine-render/src/vfx_engine.rs`
- **Interfaces**: `VfxEngine::apply_particle_system(spec)`, `VfxEngine::render_volumetric_mist()`
- **Risk Level**: Medium

### Module 24 — Color Grading & ACES Suite
- **Purpose**: Primary/secondary color wheels, ACEScg color space transformations, 3D LUT grading (`ColorGradeParams`, `AcesTransform`).
- **Dependencies**: Module 22 (Render Compositor).
- **Owning Crate**: `packages/sira-engine-render`
- **Target File**: `packages/sira-engine-render/src/color_suite.rs`
- **Interfaces**: `ColorSuiteEngine::apply_aces_grading(frame, lut_params)`
- **Risk Level**: Medium

---

## 3. Dependency Graph & Execution Order

```
[Module 17 & 18] ──> Module 19 (3D Scene Composition Engine)
                           │
                           ▼
Module 04 ─────────> Module 20 (Timeline NLE Engine)
                           │
             ┌─────────────┴─────────────┐
             ▼                           ▼
Module 21 (Multi-Track Audio)   Module 22 (Render Compositor Engine)
                                         │
                           ┌─────────────┴─────────────┐
                           ▼                           ▼
                   Module 23 (VFX Engine)     Module 24 (Color & ACES Suite)
```

---

## 4. Governance & Safety Rules

1. **Modules 00–18 Protection**: Existing source files in `packages/sira-engine-story`, `packages/sira-engine-character`, `packages/sira-engine-director`, `packages/sira-engine-cinematography` will remain 100% untouched and protected.
2. **Tag Governance**: Git completion tags `module-19-complete` through `module-24-complete` will only be created after empirical test passes.
3. **Module 61**: **NOT CREATED**. All changes fit 100% inside certified architecture.

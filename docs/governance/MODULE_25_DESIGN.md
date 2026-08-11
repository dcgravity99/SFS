# MODULE 25 DESIGN SPECIFICATION: RENDERING ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 25 establishes the **Rendering Engine** (`sira-engine-render`) for **Siragugal Film Studio**. It implements AI video frame synthesis dispatch, zero-copy frame buffer compositing (`sira_hal` shared memory ring buffers), depth pass blending, spatial upscaling (Real-ESRGAN / R-ESRGAN tile processing), render job checkpointing (`SIRA-7009`), and video container packaging specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or application feature logic.

---

## 2. Module Responsibilities & Core Features

1. **AI Frame Synthesis Dispatcher**: Route frame generation requests to AI Providers (`sira_ai_provider`) via VRAM lease allocations (`resource_manager`).
2. **Zero-Copy Frame Buffer Compositor**: Composite multi-layer video tracks, text overlays, and depth maps using `sira_hal` zero-copy Shared Memory ring buffers (`0.0 ms` copy overhead).
3. **Spatial Upscaler & Tile Processor**: Tile large 4K/8K frame passes to fit within VRAM constraints without memory exhaustion (`SIRA-2015`).
4. **Render Checkpoint Manager**: Save frame-accurate render checkpoints (`SIRA-7009`) supporting render job pause, resume, and crash recovery.
5. **Video Container Exporter**: Package raw RGB/YUV frame buffers into ProRes 422 HQ and H.264/HEVC container streams.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 24 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_hal`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`, `sira_engine_audio`, `sira_engine_timeline`, `resource_manager`, `cache_manager`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 24](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_24_COMPLETION.md).

---

## 4. Public Interfaces

Module 25 exposes public rendering engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_render)
pub struct RenderEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderJobSpec {
    pub render_job_id: String,
    pub timeline_id: String,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub target_fps: f32,
    pub codec: String, // ProRes422HQ, H264, HEVC
    pub start_frame: u64,
    pub end_frame: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderProgressSnapshot {
    pub render_job_id: String,
    pub completed_frames: u64,
    pub total_frames: u64,
    pub current_fps: f32,
    pub eta_seconds: f32,
}

impl RenderEngine {
    pub fn submit_render_job(spec: RenderJobSpec) -> SiraResult<String>;
    pub fn get_job_progress(job_id: &str) -> SiraResult<RenderProgressSnapshot>;
    pub fn cancel_render_job(job_id: &str) -> SiraResult<()>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 25 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-render/          # Rust Rendering Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & RenderEngine API
            ├── dispatcher.rs       # AI frame synthesis dispatcher
            ├── compositor.rs       # Zero-copy frame buffer compositor (sira_hal)
            ├── upscaler.rs         # Spatial upscaler & tile processor
            ├── checkpoint.rs       # Render checkpoint manager (SIRA-7009)
            └── container.rs        # ProRes / H.264 video container exporter
```

---

## 6. Testing & Validation Strategy

1. **Render Job Submission Test**: Submit 100-frame render job; verify `RenderJobId` is enqueued and VRAM lease acquired.
2. **Zero-Copy Frame Buffer Compositing Test**: Composite 2 frame layers in Shared Memory; verify memory address offsets without memory copy overhead.
3. **Render Checkpoint Pause/Resume Test**: Pause job at frame 40; resume job; verify rendering resumes cleanly at frame 40 without re-rendering frames 0-39.

---

## 7. Acceptance Criteria

Module 25 is accepted when:
1. `packages/sira-engine-render` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Render job submission, zero-copy frame compositing, and render checkpointing pass 100% of unit tests.
3. Zero UI or application feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 25: Rendering Engine**.
> 2. Upon your explicit approval, I will execute Module 25 implementation (`packages/sira-engine-render`).

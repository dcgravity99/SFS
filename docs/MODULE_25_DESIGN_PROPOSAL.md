# SIRAGUGAL FILM STUDIO — MODULE 25 DESIGN PROPOSAL
## MULTI-CAMERA CONTROLLER (`sira-engine-cinematography`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `6f134ee`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-cinematography`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary & Objective

Module 25 introduces the **Multi-Camera Controller** to `packages/sira-engine-cinematography`. In virtual film production and 3D AI animation, scenes are frequently recorded simultaneously from multiple vantage points (e.g. Master Wide, Over-the-Shoulder, Tight Close-Up, Tracking B-Cam). 

The `MultiCameraController` provides multi-angle camera registration, live-cut timecode switching, transition management (Cut, Dissolve, Fade), and automated cut-track export for consumption by the Timeline NLE Engine (Module 20) and Render Compositor (Module 22).

---

## 2. Scope & Non-Goals

- **In-Scope**:
  - `MultiCameraController` struct and camera angle specification (`CameraAngleSpec`).
  - Live-cut recording and timecode tracking (`CameraCutEvent`, `MultiCamCutTrack`).
  - Switching logic between camera rigs with zero-latency transform lookups.
  - Export of camera cut sequences to timeline tracks.
- **Non-Goals**:
  - GPU rasterization or raytracing (handled by Module 22 `layer_compositor`).
  - Direct disk storage of `.sfsp` files (handled by Module 29 `sfsp_engine`).

---

## 3. Architecture & Data Structures

### Data Contracts (`packages/sira-engine-cinematography/src/multicam.rs`)

```rust
use serde::{Deserialize, Serialize};
use sira_types::SiraResult;
use crate::optics::CameraOptics;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraAngleSpec {
    pub camera_id: String,
    pub label: String, // e.g., "A Cam - Master Wide", "B Cam - Close-Up Hero"
    pub optics: CameraOptics,
    pub initial_position: [f32; 3],
    pub initial_rotation: [f32; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraCutEvent {
    pub cut_id: String,
    pub camera_id: String,
    pub timecode_seconds: f64,
    pub transition_type: String, // "Cut", "Dissolve", "Wipe"
    pub transition_duration_seconds: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultiCamCutTrack {
    pub track_id: String,
    pub cuts: Vec<CameraCutEvent>,
}

pub struct MultiCameraController {
    cameras: Vec<CameraAngleSpec>,
    active_camera_id: Option<String>,
    cut_history: Vec<CameraCutEvent>,
}
```

---

## 4. Public API Interfaces

```rust
impl MultiCameraController {
    pub fn new() -> Self;
    pub fn add_camera_angle(&mut self, spec: CameraAngleSpec) -> SiraResult<String>;
    pub fn switch_active_camera(&mut self, camera_id: &str, timecode_seconds: f64) -> SiraResult<CameraCutEvent>;
    pub fn get_active_camera(&self) -> SiraResult<CameraAngleSpec>;
    pub fn export_cut_track(&self) -> SiraResult<MultiCamCutTrack>;
}
```

---

## 5. Error Handling & Type Safety

- Uses `SiraResult<T>` pattern matching (`match`) to guarantee zero `E0277` compiler errors.
- Error Code: `SiraErrorCode::UnknownSystemError` when referencing invalid camera IDs.

---

## 6. Offline-First, Security & Cross-Platform Design

- **Offline-First**: 100% in-memory calculations without external network API calls.
- **Security**: Validates camera ID strings to prevent memory exhaustion or invalid state corruption.
- **Cross-Platform**: Pure Rust data structures compiled natively for macOS Apple Silicon (`aarch64-apple-darwin`) and Windows/Linux (`x86_64`).

---

## 7. Files Expected to Change (Upon Authorization)

- `[NEW] packages/sira-engine-cinematography/src/multicam.rs`
- `[MODIFY] packages/sira-engine-cinematography/src/lib.rs` (Export `pub mod multicam;`)

---

```text
MODULE 25 DESIGN STATUS = COMPLETE & PROPOSED
SOURCE CODE MODIFICATIONS = NONE (0 files created/modified)
GOVERNANCE STOP = ACTIVE (Awaiting Project Owner Approval)
```

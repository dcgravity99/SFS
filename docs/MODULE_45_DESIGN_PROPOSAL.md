# SIRAGUGAL FILM STUDIO — MODULE 45 DESIGN PROPOSAL
## VIRTUAL CAMERA / CINEMATIC CAMERA AUTOMATION ENGINE (`sira-engine-cinematography`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `90f868a`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-cinematography`  
**Target Module File**: `packages/sira-engine-cinematography/src/virtual_cam.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 45 introduces procedural virtual camera movements (Dolly, Crane, Pan, Steadicam), automated focal length tracking, and depth-of-field focus targets to `packages/sira-engine-cinematography`.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-engine-cinematography`
- **Target File**: `packages/sira-engine-cinematography/src/virtual_cam.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sira-engine-cinematography::multicam` (Module 25), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: Camera transform interpolation (Spline curves), depth-of-field auto-focus calculation.
- **Non-Responsibilities**: Raster GPU frame compositing (Module 22).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraTrajectoryPoint {
    pub timestamp_seconds: f64,
    pub position: [f32; 3],
    pub target_look_at: [f32; 3],
    pub focal_length_mm: f32,
    pub aperture_fstop: f32,
}

pub struct VirtualCameraEngine;
```

## 6. Public APIs
```rust
impl VirtualCameraEngine {
    pub fn new() -> Self;
    pub fn generate_camera_path(&self, start: [f32; 3], end: [f32; 3], duration: f32) -> SiraResult<Vec<CameraTrajectoryPoint>>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-engine-cinematography/src/virtual_cam.rs`
- `[MODIFY] packages/sira-engine-cinematography/src/lib.rs`

---

```text
MODULE 45 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

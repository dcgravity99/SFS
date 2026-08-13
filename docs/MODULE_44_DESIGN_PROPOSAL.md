# SIRAGUGAL FILM STUDIO — MODULE 44 DESIGN PROPOSAL
## AI MOTION CAPTURE / BODY PERFORMANCE RETARGETING ENGINE (`sira-engine-actor`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `90f868a`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-actor`  
**Target Module File**: `packages/sira-engine-actor/src/mocap_retarget.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 44 introduces motion capture (BVH/FBX) skeleton parsing, joint hierarchy mapping, and real-time body performance retargeting to `packages/sira-engine-actor`.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-engine-actor`
- **Target File**: `packages/sira-engine-actor/src/mocap_retarget.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sira-engine-actor` (Module 12), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: Skeleton joint mapping, FK/IK retargeting solver, motion smoothing filters.
- **Non-Responsibilities**: Facial BlendShape generation (Module 43).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MocapJointTransform {
    pub joint_name: String,
    pub translation: [f32; 3],
    pub rotation_quaternion: [f32; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MocapFrame {
    pub frame_index: u32,
    pub timestamp_seconds: f64,
    pub joint_transforms: Vec<MocapJointTransform>,
}

pub struct MocapRetargetEngine;
```

## 6. Public APIs
```rust
impl MocapRetargetEngine {
    pub fn new() -> Self;
    pub fn retarget_mocap_data(&self, actor_id: &str, mocap_file_path: &str) -> SiraResult<Vec<MocapFrame>>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-engine-actor/src/mocap_retarget.rs`
- `[MODIFY] packages/sira-engine-actor/src/lib.rs`

---

```text
MODULE 44 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

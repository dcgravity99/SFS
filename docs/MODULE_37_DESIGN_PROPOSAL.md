# SIRAGUGAL FILM STUDIO — MODULE 37 DESIGN PROPOSAL
## ADVANCED COLOR GRADING & LOOK DEVELOPMENT ENGINE (`sira-engine-render`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `90f868a`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-render`  
**Target Module File**: `packages/sira-engine-render/src/color_grade.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 37 introduces advanced color management, 3D LUT (Look-Up Table) application, CDL (Color Decision List) processing, and shot-to-shot color matching to `packages/sira-engine-render`.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-engine-render`
- **Target File**: `packages/sira-engine-render/src/color_grade.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sira-engine-render::color_suite` (Module 22), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: CDL evaluation (slope, offset, power, saturation), 3D LUT parsing & interpolation, color space transform (Rec.709, ACEScg, DCI-P3).
- **Non-Responsibilities**: Layer compositing (handled by `layer_compositor.rs`).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorGradeSpec {
    pub grade_id: String,
    pub slope: [f32; 3],
    pub offset: [f32; 3],
    pub power: [f32; 3],
    pub saturation: f32,
    pub lut_file_path: Option<String>,
}

pub struct AdvancedColorGradeEngine;
```

## 6. Public APIs
```rust
impl AdvancedColorGradeEngine {
    pub fn new() -> Self;
    pub fn apply_color_grade(&self, spec: &ColorGradeSpec) -> SiraResult<bool>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-engine-render/src/color_grade.rs`
- `[MODIFY] packages/sira-engine-render/src/lib.rs`

---

```text
MODULE 37 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

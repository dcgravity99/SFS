# SIRAGUGAL FILM STUDIO — MODULE 43 DESIGN PROPOSAL
## AI CHARACTER PERFORMANCE / FACIAL ANIMATION & LIP-SYNC ENGINE (`sira-engine-character`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `90f868a`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-character`  
**Target Module File**: `packages/sira-engine-character/src/facial_anim.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 43 introduces dialogue-driven facial performance synthesis, viseme keyframing, and facial BlendShape driver generation to `packages/sira-engine-character`.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-engine-character`
- **Target File**: `packages/sira-engine-character/src/facial_anim.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sira-engine-audio::adr` (Module 26), `sira-engine-character::profile` (Module 11), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: Phoneme-to-viseme mapping, facial expression intensity curves, BlendShape weight generation.
- **Non-Responsibilities**: Full body skeletal retargeting (Module 44).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisemeKeyframe {
    pub timestamp_seconds: f32,
    pub viseme_id: String, // "A_E_I", "O_U", "B_M_P", etc.
    pub weight: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FacialPerformanceSpec {
    pub character_id: String,
    pub dialogue_audio_path: String,
    pub visemes: Vec<VisemeKeyframe>,
}

pub struct FacialAnimationEngine;
```

## 6. Public APIs
```rust
impl FacialAnimationEngine {
    pub fn new() -> Self;
    pub fn generate_facial_performance(&self, character_id: &str, audio_path: &str) -> SiraResult<FacialPerformanceSpec>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-engine-character/src/facial_anim.rs`
- `[MODIFY] packages/sira-engine-character/src/lib.rs`

---

```text
MODULE 43 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

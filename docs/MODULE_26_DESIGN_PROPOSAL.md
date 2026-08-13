# SIRAGUGAL FILM STUDIO — MODULE 26 DESIGN PROPOSAL
## AI DUBBING & AUTOMATED DIALOG REPLACEMENT (ADR) ENGINE (`sira-engine-audio`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `5a2bbbe`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-audio`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary & Objective

Module 26 introduces the **AI Dubbing & Automated Dialog Replacement (ADR) Engine** to `packages/sira-engine-audio`. In commercial filmmaking and international distribution, film soundtracks are dubbed into multiple languages (with Tamil `ta-IN` as the primary studio locale) while matching the exact lip movements, timing, and emotional cadence of original actors.

The `AdrDubbingEngine` provides automated multilingual dialogue alignment, sub-frame viseme lip-sync timestamp generation, and acoustic pitch matching between original and dubbed speech tracks.

---

## 2. Scope & Non-Goals

- **In-Scope**:
  - `AdrDubbingEngine` struct and `DubbingTargetSpec` configuration.
  - Sub-frame viseme lip-sync marker generation (`LipSyncTimestampMarker`).
  - Dialogue audio time-stretching and alignment (`DubbedAudioTrack`).
  - Alignment validation reporting (`AdrAlignmentReport`).
- **Non-Goals**:
  - Neural network model training (uses local models via `sira-ai-provider`).
  - Multi-channel surround mixing (handled by Module 21 `multitrack_mixer`).

---

## 3. Architecture & Data Structures

### Data Contracts (`packages/sira-engine-audio/src/adr.rs`)

```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};
use crate::voice::DialogueSegment;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DubbingTargetSpec {
    pub session_id: String,
    pub target_language_code: String, // e.g. "ta-IN", "en-US", "hi-IN"
    pub target_character_id: String,
    pub voice_model_id: String,
    pub preserve_pitch: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LipSyncTimestampMarker {
    pub marker_id: String,
    pub phoneme_viseme_code: String, // Viseme ID for 3D blendshape morphing
    pub timestamp_seconds: f64,
    pub duration_seconds: f32,
    pub intensity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DubbedAudioTrack {
    pub track_id: String,
    pub original_segment_id: String,
    pub language_code: String,
    pub sample_rate_hz: u32,
    pub audio_data_pcm: Vec<f32>,
    pub duration_seconds: f32,
    pub markers: Vec<LipSyncTimestampMarker>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdrAlignmentReport {
    pub alignment_score: f32, // 0.0 to 1.0
    pub duration_delta_seconds: f32,
    pub lip_sync_confidence: f32,
    pub passes_broadcast_spec: bool,
}

pub struct AdrDubbingEngine;
```

---

## 4. Public API Interfaces

```rust
impl AdrDubbingEngine {
    pub fn new() -> Self;
    pub fn align_dubbed_dialogue(
        &self,
        original_segment: &DialogueSegment,
        dubbed_text: &str,
        spec: &DubbingTargetSpec,
    ) -> SiraResult<DubbedAudioTrack>;

    pub fn generate_lip_sync_markers(
        &self,
        dubbed_track: &DubbedAudioTrack,
    ) -> SiraResult<Vec<LipSyncTimestampMarker>>;

    pub fn compute_adr_alignment_report(
        &self,
        original: &DialogueSegment,
        dubbed: &DubbedAudioTrack,
    ) -> SiraResult<AdrAlignmentReport>;
}
```

---

## 5. Offline-First, Tamil-First (`ta-IN`) & Security

- **Tamil-First (`ta-IN`)**: Includes native support for Tamil phoneme-to-viseme mapping for regional lip synchronization.
- **Offline-First**: Executes 100% locally without cloud API dependencies.
- **Security**: Validates audio buffer sizes to prevent memory overflow.

---

## 6. Files Expected to Change (Upon Authorization)

- `[NEW] packages/sira-engine-audio/src/adr.rs`
- `[MODIFY] packages/sira-engine-audio/src/lib.rs` (Export `pub mod adr;`)

---

```text
MODULE 26 DESIGN STATUS = COMPLETE & PROPOSED
SOURCE CODE MODIFICATIONS = NONE (0 files created/modified)
GOVERNANCE STOP = ACTIVE (Awaiting Project Owner Approval)
```

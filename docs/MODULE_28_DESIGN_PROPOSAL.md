# SIRAGUGAL FILM STUDIO — MODULE 28 DESIGN PROPOSAL
## SPECIAL EFFECTS (SFX) SOUND LIBRARY ENGINE (`sira-engine-audio`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `5105097`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-audio`  
**Target Module File**: `packages/sira-engine-audio/src/sfx.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Purpose & Scope

Module 28 introduces the **Special Effects (SFX) Sound Library Engine** to `packages/sira-engine-audio`. In professional cinematic post-production and AI animated filmmaking, high-quality audio design requires cataloging, searching, positioning, and synchronizing sound effects (Foley, risers, impacts, environmental ambiance, and Tamil mass stunt cues) across multi-track audio timelines.

The `SfxLibraryEngine` manages SFX asset metadata, category indexing, timeline cue placement, gain/pan automation, and loudness normalization.

---

## 2. Responsibilities & Non-Responsibilities

- **In-Scope**:
  - `SfxLibraryEngine` struct and `SfxSoundAsset` metadata specifications.
  - SFX category indexing (`SfxCategory` including regional `TamilMassStuntCue`).
  - Search and filter queries (`SfxSearchQuery`).
  - Cue placement modeling (`SfxPlacementCue`) with timecode, gain (dB), pan (-1.0 to +1.0), and fading.
  - Cue validation and loudness normalization metadata (LUFS).
- **Non-Goals**:
  - Speech synthesis / ADR (handled by Module 26 `adr.rs`).
  - Multi-channel surround render engine (handled by Module 21 `multitrack_mixer`).
  - Video frame compositing (handled by Module 22 `layer_compositor`).

---

## 3. Architecture & Data Structures

### Data Contracts (`packages/sira-engine-audio/src/sfx.rs`)

```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SfxCategory {
    ActionImpact,
    FoleyFootsteps,
    CinematicsRiser,
    EnvironmentAmbience,
    SciFiWeapons,
    TamilMassStuntCue, // Regional Tamil Cinema Stunt/Fight SFX
    Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SfxSoundAsset {
    pub asset_id: String,
    pub name: String,
    pub category: SfxCategory,
    pub tags: Vec<String>,
    pub duration_seconds: f32,
    pub sample_rate_hz: u32,
    pub channel_count: u16, // 1 = Mono, 2 = Stereo, 6 = 5.1 Surround
    pub file_path: String,
    pub loudness_lufs: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SfxPlacementCue {
    pub cue_id: String,
    pub asset_id: String,
    pub start_timecode_seconds: f64,
    pub duration_seconds: f32,
    pub gain_db: f32,
    pub pan_lr: f32, // -1.0 (Left) to +1.0 (Right)
    pub fade_in_seconds: f32,
    pub fade_out_seconds: f32,
    pub is_looping: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SfxSearchQuery {
    pub category: Option<SfxCategory>,
    pub tag_filter: Option<String>,
    pub max_duration_seconds: Option<f32>,
    pub min_sample_rate_hz: Option<u32>,
}

#[derive(Default)]
pub struct SfxLibraryEngine {
    assets: Vec<SfxSoundAsset>,
}
```

---

## 4. Public API Interfaces

```rust
impl SfxLibraryEngine {
    pub fn new() -> Self;
    pub fn register_sfx_asset(&mut self, asset: SfxSoundAsset) -> SiraResult<String>;
    pub fn search_sfx_assets(&self, query: &SfxSearchQuery) -> SiraResult<Vec<SfxSoundAsset>>;
    pub fn create_placement_cue(
        &self,
        asset_id: &str,
        start_seconds: f64,
        gain_db: f32,
    ) -> SiraResult<SfxPlacementCue>;

    pub fn validate_cue_placement(&self, cue: &SfxPlacementCue) -> SiraResult<bool>;
}
```

---

## 5. Offline-First, Security & Regional Localization

- **Tamil Mass Stunt Cues**: Native `TamilMassStuntCue` category for regional action film punch impacts and punchy sound stingers.
- **Offline-First**: Operates 100% in-memory and against local SQLite asset DB (`asset-db`).
- **Security**: Validates asset paths to prevent directory traversal.

---

## 6. Files Expected to Change (Upon Authorization)

- `[NEW] packages/sira-engine-audio/src/sfx.rs`
- `[MODIFY] packages/sira-engine-audio/src/lib.rs` (Export `pub mod sfx;`)

---

```text
MODULE 28 DESIGN STATUS = COMPLETE & PROPOSED
SOURCE FILE MODIFICATIONS = NONE (0 files created/modified)
COMMITS / PUSHES = NONE
CARGO DEPENDENCY CHANGES = NONE
MODULE 30 = NOT STARTED
MODULES 00–27 AND 29 = PRESERVED
GOVERNANCE STOP = ACTIVE
```

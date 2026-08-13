# SIRAGUGAL FILM STUDIO — MODULE 30 DESIGN PROPOSAL
## MASTER MEDIA EXPORTER & PACKAGER (`sira-engine-packaging`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `cee323a`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-packaging`  
**Target Module File**: `packages/sira-engine-packaging/src/exporter.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary & Objective

Module 30 introduces the **Master Media Exporter & Packager** to `packages/sira-engine-packaging`. In commercial theatrical distribution, streaming (OTT), and archival preservation, film projects require orchestrating final video compositing, multi-track audio mixing, subtitle sidecar generation, codec encoding, Ed25519 digital signing, and delivery package bundling.

The `MasterMediaExporterEngine` orchestrates delivery profiles (ProRes Master Archive, Broadcast Master DCP, Web H.264, Social Media Vertical), containerization, checksum verification, and `.sfsp` package manifest generation (`sfsp_version = "2.0.0"`).

---

## 2. Authoritative Package Boundary

- **Target Package**: `packages/sira-engine-packaging`
- **Reasoning**: `sira-engine-packaging` is the authoritative Level-4 orchestration package depending on `sira_engine_timeline`, `sira_engine_render`, `sira_engine_audio`, `sira_engine_story`, `sfsp_engine`, `ed25519-dalek`, and `uuid`.
- **Existing Files**: `bundler.rs`, `compression.rs`, `signature.rs`, `validator.rs`, `lib.rs`.
- **Target File for Implementation**: `packages/sira-engine-packaging/src/exporter.rs`.

---

## 3. Existing Architecture Analysis

Module 30 consumes existing certified contracts:
- `sira_engine_timeline` (Module 20: NLE Timeline Tracks)
- `sira_engine_render` (Module 22: Layer Compositor & Color Suite)
- `sira_engine_audio` (Modules 21, 26, 28: Multitrack Mixer, ADR Engine, SFX Library Engine)
- `sira_engine_story` (Module 27: Subtitle Generator Engine `.srt` / `.vtt`)
- `sfsp_engine` (Module 29: SFS Project Container format `1.0.0`)
- `ed25519-dalek` (Ed25519 cryptographic signatures)

---

## 4. Module 30 Responsibilities & Non-Responsibilities

- **In-Scope**:
  - Export request orchestration (`ExportRequest`, `MasterMediaExporterEngine`).
  - Delivery profile selection (`DeliveryProfile`).
  - Codec & container assembly (MP4, MOV, ProRes, Subtitle Sidecars).
  - SHA-256 checksum computation & Ed25519 digital signature signing.
  - Export manifest generation (`ExportManifest` with `sfsp_version = "2.0.0"`).
- **Non-Goals**:
  - Timeline editing (handled by Module 20).
  - GPU shader compositing (handled by Module 22).
  - Audio TTS / dialogue generation (handled by Modules 13 & 26).
  - Subtitle parsing (handled by Module 27).

---

## 5. Architecture & Logical Flow

```
SfspProject
   │
   ▼
ExportRequest (Delivery Profile: ProRes Master / Web H.264 / Broadcast / Subtitles)
   │
   ├──────> Timeline / Render Assembly (sira-engine-timeline & sira-engine-render)
   ├──────> Audio Master Integration (sira-engine-audio / multitrack / adr / sfx)
   └──────> Subtitle Sidecar Assembly (sira-engine-story / subtitles)
   │
   ▼
Media Encoding & Container Packaging (MP4 / MOV / ProRes / Subtitles)
   │
   ▼
SHA-256 Checksum Computation & Ed25519 Digital Signing
   │
   ▼
Final Export Manifest & Package Generation (ExportManifest, sfsp_version = "2.0.0")
```

---

## 6. Proposed Data Contracts (`packages/sira-engine-packaging/src/exporter.rs`)

```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryProfile {
    ProResMasterArchive,
    BroadcastMasterDcp,
    WebH264Mp4,
    SocialMediaVertical,
    SubtitleSidecarOnly,
    AudioMixOnly,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportRequest {
    pub request_id: String,
    pub project_path: String,
    pub output_directory: String,
    pub profile: DeliveryProfile,
    pub custom_resolution: Option<[u32; 2]>,
    pub custom_frame_rate_fps: Option<f32>,
    pub sign_package: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportArtifact {
    pub artifact_id: String,
    pub media_path: String,
    pub container_format: String,
    pub video_codec: String,
    pub audio_codec: String,
    pub duration_seconds: f64,
    pub sha256_checksum: String,
    pub ed25519_signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportManifest {
    pub export_id: String,
    pub sfsp_version: String, // "2.0.0"
    pub profile_name: String,
    pub created_at_utc: String,
    pub artifacts: Vec<ExportArtifact>,
}

#[derive(Default)]
pub struct MasterMediaExporterEngine;
```

---

## 7. Public API Contracts

```rust
impl MasterMediaExporterEngine {
    pub fn new() -> Self;
    pub fn create_export_request(&self, request: &ExportRequest) -> SiraResult<ExportManifest>;
    pub fn validate_export_manifest(&self, manifest: &ExportManifest) -> SiraResult<bool>;
}
```

---

## 8. Security, Path Validation & Determinism

- **Path Security**: Canonical path checking (`validate_canonical_path`) prevents directory traversal (`..`) attacks.
- **Determinism**: Reproducible manifest serialization and stable SHA-256 / Ed25519 checksums.

---

## 9. Future File-Level Implementation Plan

Upon authorization:
- `[NEW] packages/sira-engine-packaging/src/exporter.rs`
- `[MODIFY] packages/sira-engine-packaging/src/lib.rs` (Export `pub mod exporter; pub use exporter::*;`)

---

## 10. Non-Interference Matrix & Governance Declaration

```text
MODULE 30 DESIGN = PROPOSED ONLY
MODULE 30 IMPLEMENTATION = NOT STARTED
SOURCE CODE CHANGES = NONE
COMMITS = NONE
PUSHES = NONE
DEPENDENCY CHANGES = NONE
MODULES 00–29 = PRESERVED
MODULE 31+ = NOT STARTED
GOVERNANCE STOP = ACTIVE
```

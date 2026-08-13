# SIRAGUGAL FILM STUDIO — MODULE 27 DESIGN PROPOSAL
## SUBTITLE & CLOSED CAPTION GENERATOR (`sira-engine-story`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `37068e5`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-story`  
**Target Module File**: `packages/sira-engine-story/src/subtitles.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Purpose & Scope

Module 27 introduces the **Subtitle & Closed Caption Generator** to `packages/sira-engine-story`. In commercial film distribution, accessible subtitling and closed captioning in multiple languages (with primary Tamil `ta-IN` localization support) are mandatory for theater projection, streaming broadcast (OTT), and archival compliance.

The `SubtitleGeneratorEngine` consumes script dialogue blocks and AI voice timing data to generate broadcast-compliant SubRip (`.srt`), WebVTT (`.vtt`), and TTML caption files.

---

## 2. Responsibilities & Non-Responsibilities

- **In-Scope**:
  - `SubtitleGeneratorEngine` struct and `SubtitleTrackSpec` configuration.
  - Subtitle segmenting and line wrapping ($\le 37$ characters per line, $\le 2$ lines per caption).
  - Reading speed validation (Characters Per Second - CPS $\le 17.0$).
  - Full UTF-8 Tamil (`ta-IN`) Unicode script support.
  - Export to `.srt` (SubRip) and `.vtt` (WebVTT) formats.
- **Non-Goals**:
  - Video frame rendering / caption burn-in (handled by Module 22 `layer_compositor` / Module 30 `sira-engine-packaging`).
  - Audio speech recognition (handled by `sira-ai-provider`).

---

## 3. Architecture & Data Structures

### Data Contracts (`packages/sira-engine-story/src/subtitles.rs`)

```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};
use crate::fountain::DialogueBlock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleTrackSpec {
    pub track_id: String,
    pub language_code: String, // e.g. "ta-IN", "en-US", "hi-IN"
    pub max_characters_per_line: usize, // Default: 37
    pub max_lines_per_caption: usize,   // Default: 2
    pub max_cps: f32,                   // Default: 17.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleSegment {
    pub sequence_number: usize,
    pub start_timecode_ms: u64,
    pub end_timecode_ms: u64,
    pub speaker_name: Option<String>,
    pub text: String,
    pub language_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub track_id: String,
    pub language_code: String,
    pub segments: Vec<SubtitleSegment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleValidationReport {
    pub is_compliant: bool,
    pub total_captions: usize,
    pub total_duration_ms: u64,
    pub max_observed_cps: f32,
    pub violations: Vec<String>,
}

pub struct SubtitleGeneratorEngine;
```

---

## 4. Public API Interfaces

```rust
impl SubtitleGeneratorEngine {
    pub fn new() -> Self;
    pub fn create_subtitle_track(
        &self,
        spec: &SubtitleTrackSpec,
        dialogues: &[DialogueBlock],
    ) -> SiraResult<SubtitleTrack>;

    pub fn export_to_srt(&self, track: &SubtitleTrack) -> SiraResult<String>;
    pub fn export_to_webvtt(&self, track: &SubtitleTrack) -> SiraResult<String>;
    pub fn validate_compliance(
        &self,
        track: &SubtitleTrack,
        spec: &SubtitleTrackSpec,
    ) -> SiraResult<SubtitleValidationReport>;
}
```

---

## 5. Multilingual & Tamil (`ta-IN`) Unicode Handling

- **Tamil Script (`ta-IN`)**: Preserves full UTF-8 Unicode grapheme clusters for Tamil text (e.g. `வணக்கம்`, `சிறகுகள்`).
- **Timecode Formatting**: 
  - SRT Format: `HH:MM:SS,mmm` (e.g. `01:02:15,400 --> 01:02:18,200`)
  - WebVTT Format: `HH:MM:SS.mmm` (e.g. `01:02:15.400 --> 01:02:18.200`)

---

## 6. Files Expected to Change (Upon Authorization)

- `[NEW] packages/sira-engine-story/src/subtitles.rs`
- `[MODIFY] packages/sira-engine-story/src/lib.rs` (Export `pub mod subtitles;`)

---

```text
MODULE 27 DESIGN STATUS = COMPLETE & PROPOSED
SOURCE CODE MODIFICATIONS = NONE (0 files created/modified)
GOVERNANCE STOP = ACTIVE (Awaiting Project Owner Approval)
```

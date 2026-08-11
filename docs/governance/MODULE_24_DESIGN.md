# MODULE 24 DESIGN SPECIFICATION: TIMELINE ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 24 establishes the **Timeline Engine** (`sira-engine-timeline`) for **Siragugal Film Studio**. It implements multi-track non-destructive NLE (Non-Linear Editing) timeline synchronization (Video tracks, Dialogue tracks, Score tracks, Foley tracks, AI Prompt Keyframe tracks), SMPTE timecode frame-accurate positioning (`SiraTimecode`), non-destructive trim/ripple/roll/slip/slide editing calculations, track locking, and timeline serialization specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or AI video generation logic.

---

## 2. Module Responsibilities & Core Features

1. **Multi-Track NLE Timeline Synchronization**: Coordinate non-destructive timeline clips across Video, Dialogue, Music, Foley, and AI Prompt tracks.
2. **SMPTE Frame-Accurate Timecode Engine**: Align clips to exact frame boundaries using rational frame rates (`24.0`, `23.976`, `29.97`, `59.94`, `60.0` FPS) and drop-frame/non-drop-frame timecodes.
3. **Non-Destructive Trimming Calculator**: Calculate ripple, roll, slip, slide, and razor split operations without altering underlying asset files in `asset_db`.
4. **Track Mute / Solo & Lock Management**: Enforce track-level locking, muting, and solo overrides.
5. **Timeline Serialization**: Export versioned NLE timeline data structures for `sira-engine-render` (Module 25).

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 23 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`, `sira_engine_audio`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 23](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_23_COMPLETION.md).

---

## 4. Public Interfaces

Module 24 exposes public timeline engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_timeline)
pub struct TimelineEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineClip {
    pub clip_id: String,
    pub asset_id: String,
    pub track_id: String,
    pub in_timecode: String,   // SMPTE HH:MM:SS:FF
    pub out_timecode: String,  // SMPTE HH:MM:SS:FF
    pub start_timecode: String,// Timeline placement
    pub duration_frames: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineTrack {
    pub track_id: String,
    pub track_type: String, // Video, Dialogue, Music, Foley, Prompt
    pub is_locked: bool,
    pub is_muted: bool,
    pub clips: Vec<TimelineClip>,
}

impl TimelineEngine {
    pub fn create_timeline(name: &str, fps_numerator: u32, fps_denominator: u32) -> SiraResult<String>;
    pub fn add_clip(timeline_id: &str, clip: TimelineClip) -> SiraResult<()>;
    pub fn split_clip(timeline_id: &str, clip_id: &str, split_frame: u64) -> SiraResult<(TimelineClip, TimelineClip)>;
    pub fn serialize_timeline(timeline_id: &str) -> SiraResult<String>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 24 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-timeline/        # Rust Timeline Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & TimelineEngine API
            ├── timeline.rs         # Multi-track NLE timeline coordinator
            ├── timecode_sync.rs    # SMPTE frame-accurate timecode calculator
            ├── trimming.rs         # Ripple, roll, slip, slide & razor split operations
            ├── track.rs            # Track locking, muting & solo state manager
            └── exporter.rs         # Timeline serialization for Render Engine (Module 25)
```

---

## 6. Testing & Validation Strategy

1. **Timeline Creation & Frame Rate Test**: Create timeline at 23.976 FPS; verify frame duration calculations.
2. **Clip Placement & Split Test**: Add 100-frame clip; split at frame 40; verify output is two valid non-overlapping clips (frames 0-39 and 40-99).
3. **Track Locking Test**: Attempt adding clip to locked track; verify operation is rejected cleanly.

---

## 7. Acceptance Criteria

Module 24 is accepted when:
1. `packages/sira-engine-timeline` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Multi-track clip placement, SMPTE frame-accurate timecode sync, razor splitting, and track locking pass 100% of unit tests.
3. Zero UI or AI video generation feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 24: Timeline Engine**.
> 2. Upon your explicit approval, I will execute Module 24 implementation (`packages/sira-engine-timeline`).

# MODULE 23 DESIGN SPECIFICATION: AUDIO ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 23 establishes the **Audio Engine** (`sira-engine-audio`) for **Siragugal Film Studio**. It implements background score composition metadata, foley SFX sound generation abstraction, spatial 3D audio mixing (Dolby Atmos / Binaural spatial panning), audio stem track layering (Dialogue, Music, Foley, Ambient SFX), and audio loudness normalization (LUFS / EBU R128 compliance) specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or creative media rendering logic.

---

## 2. Module Responsibilities & Core Features

1. **Audio Stem Track Layering**: Manage 4 primary audio stems: `DialogueTrack`, `ScoreTrack`, `FoleyTrack`, and `AmbientTrack`.
2. **Spatial 3D Audio Mixer**: Calculate 3D spatial audio panning coordinates (`azimuth`, `elevation`, `distance_decay`) mapped to 3D scene grid positions.
3. **LUFS Loudness Normalization**: Calculate integrated loudness (LUFS) and true peak levels to enforce EBU R128 broadcast standards (-24 LUFS target).
4. **Foley & Music Composition Metadata**: Index audio asset references, tempo BPM, musical key signatures, and foley trigger timecodes.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 22 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 22](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_22_COMPLETION.md).

---

## 4. Public Interfaces

Module 23 exposes public audio engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_audio)
pub struct AudioEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioStem {
    pub stem_id: String,
    pub stem_type: String, // Dialogue, Music, Foley, Ambient
    pub sample_rate_hz: u32,
    pub channels: u16,     // 1=Mono, 2=Stereo, 6=5.1, 8=7.1
    pub volume_db: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpatialAudioParams {
    pub azimuth_degrees: f32,
    pub elevation_degrees: f32,
    pub distance_meters: f32,
    pub room_reverb_decay_sec: f32,
}

impl AudioEngine {
    pub fn create_stem(stem_type: &str, sample_rate: u32) -> SiraResult<AudioStem>;
    pub fn compute_spatial_panning(listener_pos: [f32; 3], source_pos: [f32; 3]) -> SiraResult<SpatialAudioParams>;
    pub fn normalize_lufs(input_lufs: f32, target_lufs: f32) -> SiraResult<f32>; // Returns gain adjustment dB
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 23 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-audio/           # Rust Audio Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & AudioEngine API
            ├── stems.rs            # AudioStem multi-track layering manager
            ├── spatial.rs          # Spatial 3D audio panning & binaural calculator
            ├── lufs.rs             # EBU R128 LUFS loudness normalization calculator
            └── composition.rs      # Score tempo BPM & foley trigger metadata
```

---

## 6. Testing & Validation Strategy

1. **Audio Stem Creation Test**: Create dialogue and score stems; verify sample rates (48000 Hz) and volume dB settings.
2. **Spatial 3D Panning Test**: Input listener `[0,0,0]` and source `[2,0,0]`; verify azimuth degrees equals 90.0° (right channel).
3. **LUFS Normalization Test**: Calculate gain adjustment for -18 LUFS input to -24 LUFS target (gain = -6.0 dB).

---

## 7. Acceptance Criteria

Module 23 is accepted when:
1. `packages/sira-engine-audio` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Audio stem track creation, spatial panning calculations, and LUFS gain normalization pass 100% of unit tests.
3. Zero UI or media rendering feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 23: Audio Engine**.
> 2. Upon your explicit approval, I will execute Module 23 implementation (`packages/sira-engine-audio`).

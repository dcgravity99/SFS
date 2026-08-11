# MODULE 19 DESIGN SPECIFICATION: ACTOR ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 19 establishes the **Actor Engine** (`sira-engine-actor`) for **Siragugal Film Studio**. It implements synthetic voice identity management, provider-agnostic speech synthesis abstraction, viseme lip-sync timeline generation (Preston Blair viseme mapping), multilingual dialogue mapping, emotional speech metadata indexing, pronunciation dictionaries, and audio-dialogue timeline synchronization specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or creative media rendering logic.

---

## 2. Module Responsibilities & Core Features

1. **Voice Identity & Actor Registry**: Bind synthetic voice model IDs (ElevenLabs, Coqui, XTTS, Bark) to `ActorId` handles stored in `asset_db`.
2. **Viseme Lip-Sync Generator**: Convert dialogue text and audio timing into timed viseme keyframes (`A`, `B`, `C`, `D`, `E`, `F`, `G`, `H`, `X`).
3. **Multilingual Speech Mapper**: Map character dialogue lines across international languages and phonetic pronunciation dictionaries.
4. **Emotional Speech Metadata Manager**: Index vocal emotion attributes (Pitch, Pace, Cadence, Inflexion, Excitement, Whispering).
5. **Voice Consistency Validator**: Compute acoustic spectral embedding similarity to detect voice drift across generated audio tracks.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 18 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 18](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_18_COMPLETION.md).

---

## 4. Public Interfaces

Module 19 exposes public actor engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_actor)
pub struct ActorEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisemeKeyframe {
    pub timestamp_ms: u64,
    pub viseme_code: String, // A, B, C, D, E, F, G, H, X
    pub weight: f32,         // 0.0 to 1.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActorProfile {
    pub actor_id: String,
    pub character_id: String,
    pub voice_model_id: String,
    pub pitch_offset: f32,
    pub speech_pace: f32,
    pub language_code: String,
}

impl ActorEngine {
    pub fn create_actor(character_id: &str, voice_model_id: &str) -> SiraResult<ActorProfile>;
    pub fn bind_voice(actor_id: &str, voice_model_id: &str) -> SiraResult<()>;
    pub fn generate_lipsync(audio_duration_ms: u64, speech_text: &str) -> SiraResult<Vec<VisemeKeyframe>>;
    pub fn verify_voice_consistency(target_embedding: &[f32], anchor_embedding: &[f32]) -> SiraResult<f32>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 19 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-actor/          # Rust Actor Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & ActorEngine API
            ├── actor.rs            # ActorProfile & voice registry manager
            ├── voice.rs            # Voice synthesis provider abstraction
            ├── lipsync.rs          # Preston Blair viseme lip-sync generator
            ├── dictionary.rs       # Phonetic pronunciation dictionary
            └── consistency.rs      # Acoustic spectral embedding distance validator
```

---

## 6. Testing & Validation Strategy

1. **Actor Profile & Voice Binding Test**: Bind voice model ID to `ActorId`; verify profile metadata indexes cleanly in `asset_db`.
2. **Viseme Lip-Sync Generation Test**: Input speech text and 3000ms duration; verify generator outputs ordered `VisemeKeyframe` array.
3. **Voice Consistency Distance Test**: Compare identical audio spectral vectors (similarity = 1.0); verify distance calculation.

---

## 7. Acceptance Criteria

Module 19 is accepted when:
1. `packages/sira-engine-actor` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Actor creation, viseme lip-sync generation, and voice consistency checks pass 100% of unit tests.
3. Zero UI or media rendering feature code is present.

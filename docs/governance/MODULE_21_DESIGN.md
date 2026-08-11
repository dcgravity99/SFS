# MODULE 21 DESIGN SPECIFICATION: DIRECTOR ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 21 establishes the **Director Engine** (`sira-engine-director`) for **Siragugal Film Studio**. It implements cinematic shot planning, shot list sequencing, scene actor blocking layout translation, emotional pacing evaluation, camera framing intent (Wide, Medium, Close-Up, Extreme Close-Up, Over-The-Shoulder), storyboard metadata generation, and directorial decision support specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or AI video generation logic.

---

## 2. Module Responsibilities & Core Features

1. **Cinematic Shot Plan Generator**: Translate narrative scenes (`ScriptScene`) and story beats (`StoryBeat`) into structured shot lists (`ShotPlan`).
2. **Storyboard Metadata Engine**: Generate visual storyboard metadata anchors (Shot Type, Camera Angle, Lens Length, Key Action).
3. **Scene Blocking Coordinator**: Map actor positions and camera movement vectors in 3D space (`sira_engine_scene`).
4. **Emotional Pacing Evaluator**: Calculate shot duration and cutting frequency to match scene emotional intensity.
5. **Shot Sequence Continuity Validator**: Enforce the 180-degree rule and axis of action continuity checks.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 20 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 20](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_20_COMPLETION.md).

---

## 4. Public Interfaces

Module 21 exposes public director engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_director)
pub struct DirectorEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShotPlan {
    pub shot_id: String,
    pub scene_id: usize,
    pub shot_type: String,     // Wide, Medium, CloseUp, OTS
    pub camera_angle: String,  // EyeLevel, HighAngle, LowAngle, Dutch
    pub lens_focal_length_mm: f32,
    pub duration_seconds: f32,
    pub key_action_description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryboardFrame {
    pub frame_id: String,
    pub shot_id: String,
    pub frame_index: usize,
    pub framing_summary: String,
}

impl DirectorEngine {
    pub fn create_shot_plan(scene_id: usize, prompt: &str) -> SiraResult<Vec<ShotPlan>>;
    pub fn generate_storyboard(shots: &[ShotPlan]) -> SiraResult<Vec<StoryboardFrame>>;
    pub fn validate_shot_sequence(shots: &[ShotPlan]) -> SiraResult<bool>; // Verifies 180-degree rule continuity
    pub fn evaluate_scene_pacing(shots: &[ShotPlan]) -> SiraResult<f32>;   // Returns emotional intensity index (0.0 to 1.0)
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 21 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-director/        # Rust Director Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & DirectorEngine API
            ├── shot_plan.rs        # Cinematic ShotPlan generator
            ├── storyboard.rs       # StoryboardFrame metadata generator
            ├── blocking.rs         # Scene actor blocking coordinator
            ├── pacing.rs           # Emotional pacing evaluator
            └── intent.rs           # 180-degree rule continuity validator
```

---

## 6. Testing & Validation Strategy

1. **Shot Plan Generation Test**: Input scene ID and narrative description; verify generator creates structured `ShotPlan` items.
2. **Storyboard Metadata Test**: Supply `ShotPlan` list; verify storyboard frames contain matching shot IDs and index numbers.
3. **Continuity Validator Test**: Validate 180-degree rule; verify validator detects axis jumps across camera cuts.

---

## 7. Acceptance Criteria

Module 21 is accepted when:
1. `packages/sira-engine-director` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Cinematic shot plan generation, storyboard metadata creation, and continuity checks pass 100% of unit tests.
3. Zero UI or AI video generation feature code is present.

# MODULE 18 DESIGN SPECIFICATION: CHARACTER ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 18 establishes the **Character Engine** (`sira-engine-character`) for **Siragugal Film Studio**. It implements visual character consistency management, character identity anchoring (facial feature vectors, costume reference embeddings, project-level LoRA weight bindings), character profile metadata indexing, and character visual drift verification across AI-generated video shots specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or AI video generation logic.

---

## 2. Module Responsibilities & Core Features

1. **Character Visual Identity Anchoring**: Index character facial embeddings, hairstyle vectors, body height metrics, and costume variation references.
2. **Project-Level LoRA Binding Manager**: Bind character-specific LoRA weights (`.safetensors`) to `CharacterId` handles stored in `asset_db`.
3. **Character Profile Metadata Registry**: Manage character names, age, role (Protagonist, Antagonist, Supporting), personality traits, and voice model IDs.
4. **Visual Consistency Validator**: Calculate visual feature embedding distance metrics to detect character drift between generated shots.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 17 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 17](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_17_COMPLETION.md).

---

## 4. Public Interfaces

Module 18 exposes public character engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_character)
pub struct CharacterEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub character_id: String,
    pub name: String,
    pub role: String,
    pub voice_model_id: Option<String>,
    pub lora_weight_path: Option<PathBuf>,
    pub visual_anchor_embeddings: Vec<f32>,
}

impl CharacterEngine {
    pub fn create_character(name: &str, role: &str) -> SiraResult<CharacterProfile>;
    pub fn bind_lora(character_id: &str, lora_path: &Path) -> SiraResult<()>;
    pub fn verify_visual_consistency(target_embedding: &[f32], anchor_embedding: &[f32]) -> SiraResult<f32>; // Returns similarity score (0.0 to 1.0)
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 18 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-character/      # Rust Character Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & CharacterEngine API
            ├── profile.rs          # CharacterProfile registry & metadata manager
            ├── anchors.rs          # Visual feature anchor embedding manager
            ├── lora_binding.rs     # LoRA weight file binding engine
            └── consistency.rs      # Visual embedding similarity & drift validator
```

---

## 6. Testing & Validation Strategy

1. **Character Creation & Profile Test**: Create character profile; verify `CharacterId` UUID v7 is generated and indexed in `asset_db`.
2. **LoRA Binding Test**: Bind `.safetensors` LoRA path; verify path is recorded and validated.
3. **Visual Consistency Distance Test**: Compare two identical feature vectors (similarity = 1.0); compare two orthogonal vectors (similarity = 0.0).

---

## 7. Acceptance Criteria

Module 18 is accepted when:
1. `packages/sira-engine-character` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Character profile registration, LoRA binding, and visual consistency distance checks pass 100% of unit tests.
3. Zero UI or AI video generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 18: Character Engine**.
> 2. Upon your explicit approval, I will execute Module 18 implementation (`packages/sira-engine-character`).

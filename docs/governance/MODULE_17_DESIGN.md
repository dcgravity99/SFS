# MODULE 17 DESIGN SPECIFICATION: STORY ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 17 establishes the **Story Engine** (`sira-engine-story`) for **Siragugal Film Studio**. It implements narrative screenplay breakdown (parsing standard Fountain `.fountain` and Final Draft `.fdx` script formats), story beat sheet generation (3-Act structure, Blake Snyder Save the Cat beats, Hero's Journey arcs), character dialogue extraction, and narrative act boundary identification without adding UI views or AI video generation logic.

---

## 2. Module Responsibilities & Core Features

1. **Fountain & FDX Script Parser**: Parses plain text Fountain screenplays and Final Draft XML into structured `ScriptAST` scenes, character cues, dialogue blocks, parentheticals, and action headings.
2. **Story Beat Graph Generator**: Translates script scenes into structured narrative beats (`Opening Image`, `Theme Stated`, `Catalyst`, `Break into Two`, `Midpoint`, `All Is Lost`, `Climax`, `Resolution`).
3. **Character & Dialogue Extractor**: Extracts character speech lines, word counts, and scene appearance matrices.
4. **Narrative Beat Integrity Verifier**: Validates act pacing and story structure continuity.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 16 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`), Rust `serde_json`, `quick-xml`.
- **Module Dependencies**: Depends on [Modules 01 - 16](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_16_COMPLETION.md).

---

## 4. Public Interfaces

Module 17 exposes public story engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_story)
pub struct StoryEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScriptScene {
    pub scene_number: usize,
    pub heading: String, // INT. SOUNDSTAGE A - DAY
    pub action_lines: Vec<String>,
    pub dialogue_blocks: Vec<DialogueBlock>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueBlock {
    pub character_name: String,
    pub parenthetical: Option<String>,
    pub speech_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryBeat {
    pub beat_id: String,
    pub beat_type: String, // Catalyst, Midpoint, etc.
    pub scene_ids: Vec<usize>,
    pub description: String,
}

impl StoryEngine {
    pub fn parse_fountain(script_text: &str) -> SiraResult<Vec<ScriptScene>>;
    pub fn generate_beat_sheet(scenes: &[ScriptScene]) -> SiraResult<Vec<StoryBeat>>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 17 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-story/          # Rust Story Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & StoryEngine API
            ├── fountain.rs         # Fountain script parser & AST builder
            ├── fdx.rs              # Final Draft FDX XML parser
            ├── beats.rs            # Story Beat Sheet generator
            ├── dialogue.rs         # Character dialogue extractor
            └── validator.rs        # Narrative structure integrity validator
```

---

## 6. Testing & Validation Strategy

1. **Fountain Script Parsing Test**: Parse standard Fountain script sample; verify scene headings, action lines, and character dialogue blocks extract cleanly.
2. **Story Beat Generation Test**: Supply 3-Act scene list; verify beat generator outputs ordered `StoryBeat` items matching structural beats.
3. **Dialogue Extraction Test**: Parse multi-character scene; verify character speech lines match expected word counts.

---

## 7. Acceptance Criteria

Module 17 is accepted when:
1. `packages/sira-engine-story` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Fountain screenplay parsing and beat sheet generation pass 100% of unit tests.
3. Dialogue extractions maintain exact text fidelity without data loss.
4. Zero UI or AI video generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 17: Story Engine**.
> 2. Upon your explicit approval, I will execute Module 17 implementation (`packages/sira-engine-story`).

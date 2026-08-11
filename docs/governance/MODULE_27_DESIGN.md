# MODULE 27 DESIGN SPECIFICATION: WORKFLOW AUTOMATION ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 27 establishes the **Workflow Automation Engine** (`sira-engine-workflow`) for **Siragugal Film Studio**. It implements end-to-end film production pipeline templates (Script-to-Screen DAG, Character-to-Shot pipeline, Automated Batch Rendering pipeline), sub-engine orchestration across Modules 17 through 26, automated task scheduling, and error recovery policies specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or application feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Script-to-Screen Production Pipeline Template**: Orchestrate multi-step film generation (Script breakdown -> Story beat sheet -> Character assignment -> Actor voicing -> 3D scene placement -> Director shot plan -> Cinematography optics -> Audio stems -> NLE timeline -> Video rendering).
2. **Sub-Engine DAG Execution Orchestrator**: Schedule node dependencies across all 12 SIRA sub-engines using `workflow_engine` execution contracts.
3. **Automated Batch Render Pipeline**: Batch process multi-shot scene render queues with priority scheduling (`resource_manager`).
4. **Pipeline Failure Recovery Handler**: Execute automated retry policies and fallback providers on task failures (`SIRA-5012`).

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 26 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_hal`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`, `sira_engine_audio`, `sira_engine_timeline`, `sira_engine_render`, `sira_engine_asset`, `resource_manager`, `cache_manager`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 26](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_26_COMPLETION.md).

---

## 4. Public Interfaces

Module 27 exposes public workflow automation engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_workflow)
pub struct WorkflowAutomationEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineExecutionSpec {
    pub pipeline_id: String,
    pub template_name: String, // ScriptToScreen, CharacterToShot, BatchRender
    pub script_asset_id: String,
    pub max_parallel_jobs: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineStatus {
    pub pipeline_id: String,
    pub current_step: String,
    pub progress_percentage: f32,
    pub completed_steps: Vec<String>,
    pub is_failed: bool,
}

impl WorkflowAutomationEngine {
    pub fn execute_pipeline(spec: PipelineExecutionSpec) -> SiraResult<String>;
    pub fn get_pipeline_status(pipeline_id: &str) -> SiraResult<PipelineStatus>;
    pub fn cancel_pipeline(pipeline_id: &str) -> SiraResult<()>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 27 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-workflow/       # Rust Workflow Automation Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & WorkflowAutomationEngine API
            ├── pipeline.rs         # Script-to-screen pipeline template coordinator
            ├── orchestrator.rs     # Sub-engine DAG execution orchestrator
            ├── batch.rs            # Automated batch render queue scheduler
            └── recovery.rs         # Pipeline failure recovery & retry handler
```

---

## 6. Testing & Validation Strategy

1. **Pipeline Execution Test**: Execute Script-to-Screen pipeline template; verify step ordering across all 12 SIRA sub-engines.
2. **Sub-Engine DAG Orchestration Test**: Verify DAG dependency execution prevents cyclic deadlock (`SIRA-5012`).
3. **Pipeline Failure Recovery Test**: Inject step failure; verify recovery handler executes retry policy successfully.

---

## 7. Acceptance Criteria

Module 27 is accepted when:
1. `packages/sira-engine-workflow` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Script-to-screen pipeline execution, sub-engine DAG orchestration, and failure recovery pass 100% of unit tests.
3. Zero UI or application feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 27: Workflow Automation Engine**.
> 2. Upon your explicit approval, I will execute Module 27 implementation (`packages/sira-engine-workflow`).

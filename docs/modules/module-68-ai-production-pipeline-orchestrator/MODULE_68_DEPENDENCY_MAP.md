# MODULE 68 — AI PRODUCTION PIPELINE ORCHESTRATOR ENGINE DEPENDENCY MAP

**Target Package**: `packages/sira-engine-workflow`  

---

## 1. Upstream & Downstream Dependency Matrix

```
[Module 48: Production Planning] ───────┐
                                        ├──> [MODULE 68: Pipeline Orchestrator]
[Modules 62–67: Creative Intelligence] ─┤
                                        │
[Module 61: AI Director Engine] ────────┘
                                        │
                                        ▼
                   [Downstream Render & Release Engines]
                   (Modules 22, 41, 53)
```

| Component | Nature of Dependency | Interface / Struct Consumed |
| :--- | :--- | :--- |
| **`sira_types`** | Hard Dependency | `SiraResult`, `SiraError`, `SiraErrorCode` |
| **`sira-engine-workflow` (Mod 48)** | Internal Package Dependency | `production_planner::ProductionPlanSpec` |
| **`sira-engine-director` (Mod 61)** | Logical Consumer | `DirectorDecision`, `DirectorRequest` |
| **`sira-ecosystem-engine` (Mod 55)** | Inter-Engine Orchestration | `MasterDispatcherEngine`, `MasterJobDag` |

---

## 2. Dependency Rules & Safety

1. **No Circular Dependencies**: Module 68 coordinates workflow execution without mutating upstream decision engines.
2. **Preservation of Module 61**: Module 61 remains the sole owner of director decision generation (`DirectorDecision`).
3. **Workspace Isolation**: Uses only existing workspace crates (`sira_types`, `serde`). 0 new Cargo dependencies introduced.

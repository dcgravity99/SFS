# DEPENDENCY GRAPH v2.0
**Siragugal Film Studio**  
**Document Version**: 2.0.0  
**Status**: APPROVED COMPREHENSIVE DEPENDENCY GRAPH  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Complete System Dependency Topology

```mermaid
graph TD
    subgraph Presentation & Experience
        UI["Presentation Layer (Tauri/React)"] --> Exp["Experience Layer (Module 16)"]
    end

    subgraph Phase 2 SIRA Sub-Engines
        Exp --> Story["Story Engine (M17)"]
        Exp --> Char["Character Engine (M18)"]
        Exp --> Actor["Actor Engine (M19)"]
        Exp --> Scene["Scene Engine (M20)"]
        Exp --> Dir["Director Engine (M21)"]
        Exp --> Cine["Cinematography Engine (M22)"]
        Exp --> Audio["Audio Engine (M23)"]
        Exp --> Timeline["Timeline Engine (M24)"]
        Exp --> Render["Rendering Engine (M25)"]
        Exp --> Edit["Editing Engine (M26)"]
        Exp --> Prod["Producer Engine (M27)"]
        Exp --> Orch["Orchestrator Engine (M28)"]
    end

    subgraph Phase 1 Infrastructure Foundation
        Orch --> Workflow["workflow_engine (M12)"]
        Workflow --> Plugin["plugin_runtime (M13)"]
        Plugin --> Res["resource_manager (M14)"]
        Res --> Cache["cache_manager (M15)"]
        Cache --> AIProvider["sira_ai_provider (M11)"]
        AIProvider --> Core["sira_core (M10)"]
        Core --> HAL["sira_hal (M09)"]
        HAL --> AssetDB["asset_db (M08)"]
        AssetDB --> SFSP["sfsp_engine (M07)"]
        SFSP --> Settings["sira_settings (M06)"]
        Settings --> Diag["sira_diagnostics (M05)"]
        Diag --> Config["sira_config (M04)"]
        Config --> Types["sira_types (M03)"]
    end
```

---

## 2. Dependency Rules

1. **Acyclic Invariant**: Dependencies flow strictly downward from Presentation → Phase 2 Sub-Engines → Phase 1 Infrastructure.
2. **Infrastructure Independence**: Lower-level infrastructure crates NEVER depend on Phase 2 sub-engines or presentation modules.
3. **Hardware Isolation**: AI sub-engines execute compute strictly through `sira_hal` and `sira_ai_provider`, never calling GPU drivers directly.

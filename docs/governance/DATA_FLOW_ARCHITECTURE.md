# DATA FLOW ARCHITECTURE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED DATA FLOW ARCHITECTURE SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Voice → Film End-to-End Data Flow

```mermaid
flowchart LR
    A["Raw Dialogue Audio"] --> B["Actor Engine (M19)"]
    B -->|"SpeechToText & Lip-Sync"| C["Script Breakdown Matrix"]
    C --> D["Scene Engine (M20)"]
    D -->|"3D Camera & Pose Grid"| E["Workflow Engine (M12)"]
    E -->|"DAG Execution"| F["Rendering Engine (M25)"]
    F -->|"Zero-Copy Frames"| G["NLE Timeline Output"]
```

---

## 2. Script → Film End-to-End Data Flow

```mermaid
flowchart LR
    A["Fountain Screenplay"] --> B["Story Engine (M17)"]
    B -->|"Story Beat Graph"| C["Director Engine (M21)"]
    C -->|"Shot Composition & Lenses"| D["Cinematography Engine (M22)"]
    D -->|"Render Parameters"| E["Rendering Engine (M25)"]
    E -->|"ProRes Video Track"| F["Final Movie Project"]
```

---

## 3. Save / Open Storage Pipeline Flow

```mermaid
flowchart TD
    Sub1["Project State Data"] --> Sub2["sfsp_engine Package Writer"]
    Sub2 --> Sub3["manifest.json Checksum Sync"]
    Sub3 --> Sub4["SQLite project.db WAL Commit"]
    Sub4 --> Sub5["Staged .sfsp.tmp Directory"]
    Sub5 --> Sub6["Atomic Directory Replacement"]
```

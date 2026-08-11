# MODULE 12 DESIGN SPECIFICATION: WORKFLOW GRAPH ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 12 establishes the node-based Workflow Graph Engine (`workflow-engine`) for **Siragugal Film Studio**. It implements visual non-destructive pipeline execution, strongly typed Node Contracts, canonical data port bindings, versioned workflow specifications, resource-aware DAG scheduling, deterministic execution planning, checkpoint recovery, multi-stage validation, `.sfsw` marketplace bundle compatibility, and distributed execution abstractions without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Strongly Typed Node Contracts**: Define nodes under 10 categories (`Input`, `AI`, `Media`, `Logic`, `Timeline`, `Render`, `Export`, `Utility`, `Plugin`, `System`) with mandatory metadata (version, inputs, outputs, required capability, resource requirements, cache policy).
2. **Canonical Data Contracts**: Enforce port type compatibility across 13 data types (`Story`, `SceneList`, `CharacterList`, `Dialogue`, `VoiceTrack`, `VideoClip`, `Image`, `Subtitle`, `Timeline`, `Prompt`, `Embedding`, `Metadata`, `RawBuffer`).
3. **Multi-Stage Workflow Validation**: Validate 6 integrity checks before execution:
   - *Check 1: Syntax & Edge Binding*
   - *Check 2: Port Type Compatibility*
   - *Check 3: DAG Cycle Detection (`SIRA-5012`)*
   - *Check 4: Required AI Capability Availability*
   - *Check 5: Plugin & Model Version Requirements*
   - *Check 6: Resource Conflict Analysis*
4. **Resource-Aware DAG Scheduler**: Dispatch nodes based on real-time VRAM/RAM availability, CPU core limits, AI provider health, thermal state, and priority policy.
5. **Deterministic Execution Planning**: Generate reproducible execution graphs based on node input hashes (`node_hash`).
6. **Workflow Checkpoint & Crash Recovery**: Record execution progress snapshots supporting Pause, Resume, Reboot Recovery, and Partial Re-execution.
7. **Marketplace Bundle (`.sfsw`) Format Compatibility**: Support export, import, schema versioning, and digital signatures for `.sfsw` workflow files.
8. **Distributed Execution Abstraction**: Abstract local, LAN render farm, and cloud execution backends without modifying graph definitions.

---

## 3. Node Contract & Data Binding Schema

```json
{
  "node_id": "node-storyboard-gen-01",
  "category": "AI",
  "name": "SIRA Diffusion Storyboard Node",
  "version": "1.0.0",
  "required_capability": "ImageGeneration",
  "resource_requirements": {
    "vram_mb": 8192,
    "ram_mb": 4096
  },
  "inputs": [
    { "port_name": "prompt", "data_type": "Prompt" },
    { "port_name": "character_ref", "data_type": "CharacterList" }
  ],
  "outputs": [
    { "port_name": "storyboard_stills", "data_type": "Image" }
  ],
  "cache_policy": "CacheByInputHash"
}
```

---

## 4. File Blueprint

Module 12 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── workflow-engine/            # Rust Workflow DAG Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & WorkflowGraph API
            ├── contract.rs         # NodeContract & NodeCategory definitions
            ├── types.rs            # Canonical DataContracts (Story, VideoClip, etc.)
            ├── node.rs             # WorkflowNode & port definitions
            ├── edge.rs             # WorkflowEdge & type compatibility validator
            ├── dag.rs              # Topological sorter & multi-stage validator (SIRA-5012)
            ├── scheduler.rs        # Resource-aware DAG scheduler
            ├── checkpoint.rs       # Checkpointing & crash recovery engine
            ├── cache.rs            # Deterministic execution planner & node hash cache
            ├── sfsw.rs             # Marketplace bundle (.sfsw) importer/exporter & signatures
            └── executor.rs         # Execution engine & distributed execution abstraction
```

---

## 5. Acceptance Criteria

Module 12 is accepted when:
1. `packages/workflow-engine` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Any built-in or plugin node participates cleanly in graph execution by implementing `NodeContract`, with zero changes to the Workflow Engine core.
3. Multi-stage validation catches circular dependencies (`SIRA-5012`), port type mismatches, and resource conflicts correctly.
4. `.sfsw` marketplace workflow files import, validate, sign, and export cleanly.
5. Zero application or creative feature code is present.

# SIRAGUGAL FILM STUDIO — MODULE 38 DESIGN PROPOSAL
## VISUAL EFFECTS COMPOSITING & NODE GRAPH ENGINE (`sira-engine-render`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `90f868a`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-engine-render`  
**Target Module File**: `packages/sira-engine-render/src/node_graph.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 38 introduces node-based visual effects DAG (Directed Acyclic Graph) evaluation, keying, masking, and spatial blend node graphs to `packages/sira-engine-render`.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-engine-render`
- **Target File**: `packages/sira-engine-render/src/node_graph.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sira-engine-render::vfx_engine` (Module 22), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: VFX node evaluation, DAG topological sorting, Chroma keying & alpha masking.
- **Non-Responsibilities**: Final export container packaging (Module 30).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VfxNodeSpec {
    pub node_id: String,
    pub node_type: String, // "ChromaKey", "Blur", "Transform", "Blend"
    pub inputs: Vec<String>,
    pub parameters_json: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VfxNodeGraph {
    pub graph_id: String,
    pub nodes: Vec<VfxNodeSpec>,
}

pub struct VfxNodeGraphEngine;
```

## 6. Public APIs
```rust
impl VfxNodeGraphEngine {
    pub fn new() -> Self;
    pub fn evaluate_graph(&self, graph: &VfxNodeGraph) -> SiraResult<bool>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-engine-render/src/node_graph.rs`
- `[MODIFY] packages/sira-engine-render/src/lib.rs`

---

```text
MODULE 38 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

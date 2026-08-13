# SIRAGUGAL FILM STUDIO — MODULE 32 DESIGN PROPOSAL
## AUTOMATED CLOUD / LOCAL RENDER FARM DISPATCHER & LOAD BALANCER (`sira-deployment-engine`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `36f577f`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-deployment-engine`  
**Target Module File**: `packages/sira-deployment-engine/src/render_farm.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 32 introduces the **Render Farm Dispatcher & Load Balancer** to `packages/sira-deployment-engine`. It distributes frame-rendering jobs across local CPU/GPU workers and cloud instances.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-deployment-engine`
- **Target File**: `packages/sira-deployment-engine/src/render_farm.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sira_engine_render` (Module 22), `sfsp_engine` (Module 29), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: Render job slice dispatching, node health monitoring, load balancing.
- **Non-Responsibilities**: GPU frame compositing (Module 22).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderJobSpec {
    pub job_id: String,
    pub project_path: String,
    pub start_frame: u32,
    pub end_frame: u32,
    pub priority: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderNodeStatus {
    pub node_id: String,
    pub is_online: bool,
    pub current_load_percentage: f32,
    pub active_job_id: Option<String>,
}

pub struct RenderFarmDispatcher;
```

## 6. Public APIs
```rust
impl RenderFarmDispatcher {
    pub fn new() -> Self;
    pub fn dispatch_job(&self, spec: &RenderJobSpec) -> SiraResult<String>;
    pub fn query_node_health(&self) -> SiraResult<Vec<RenderNodeStatus>>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-deployment-engine/src/render_farm.rs`
- `[MODIFY] packages/sira-deployment-engine/src/lib.rs`

---

```text
MODULE 32 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

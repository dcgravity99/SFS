# MODULE 67 — AI CREATIVE CONSISTENCY & FRANCHISE GOVERNANCE ENGINE INTERFACES

**Target Package**: `packages/sira-ecosystem-engine`  

---

## Data Contracts & Structures

```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsistencyAuditRequest {
    pub franchise_id: String,
    pub project_id: String,
    pub canon_rules: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsistencyAuditReport {
    pub audit_id: String,
    pub is_canon_compliant: bool,
    pub violations_count: u32,
    pub approval_required: bool,
    pub reasoning_trace_id: String,
}

pub struct CreativeConsistencyEngine;

impl CreativeConsistencyEngine {
    pub fn new() -> Self;
    pub fn audit_consistency(&self, request: &ConsistencyAuditRequest) -> SiraResult<ConsistencyAuditReport>;
}
```

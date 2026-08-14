# MODULE 66 — AI CINEMATIC STYLE & VISUAL LANGUAGE ENGINE INTERFACES

**Target Package**: `packages/sira-engine-cinematography`  

---

## Data Contracts & Structures

```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CinematicStyleRequest {
    pub project_id: String,
    pub director_preset_name: String,
    pub lens_focal_mm: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CinematicStyleReport {
    pub evaluation_id: String,
    pub style_match_score: f32,
    pub approval_required: bool,
    pub reasoning_trace_id: String,
}

pub struct CinematicStyleEngine;

impl CinematicStyleEngine {
    pub fn new() -> Self;
    pub fn evaluate_style(&self, request: &CinematicStyleRequest) -> SiraResult<CinematicStyleReport>;
}
```

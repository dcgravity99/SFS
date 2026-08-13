# SIRAGUGAL FILM STUDIO — MODULE 34 DESIGN PROPOSAL
## FINE-TUNING & CUSTOM LORA TRAINING PIPELINE ENGINE (`sira-ai-acceleration-engine`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `36f577f`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-ai-acceleration-engine`  
**Target Module File**: `packages/sira-ai-acceleration-engine/src/lora_training.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 34 introduces custom LoRA (Low-Rank Adaptation) fine-tuning pipelines for character and style consistency to `packages/sira-ai-acceleration-engine`.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-ai-acceleration-engine`
- **Target File**: `packages/sira-ai-acceleration-engine/src/lora_training.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sira_ai_provider` (Module 15 / Candle backend), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: LoRA weight extraction, rank-adaptation matrix computation, gradient checkpointing.
- **Non-Responsibilities**: Base model weight inference (handled by `sira-ai-provider`).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoraTrainingConfig {
    pub task_id: String,
    pub character_id: String,
    pub dataset_directory: String,
    pub rank_dim: u32,
    pub alpha_scale: f32,
    pub max_steps: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoraTrainingProgress {
    pub current_step: u32,
    pub total_steps: u32,
    pub loss: f32,
    pub output_weights_path: String,
}

pub struct LoraTrainingEngine;
```

## 6. Public APIs
```rust
impl LoraTrainingEngine {
    pub fn new() -> Self;
    pub fn start_training(&self, config: &LoraTrainingConfig) -> SiraResult<String>;
    pub fn query_training_progress(&self, task_id: &str) -> SiraResult<LoraTrainingProgress>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-ai-acceleration-engine/src/lora_training.rs`
- `[MODIFY] packages/sira-ai-acceleration-engine/src/lib.rs`

---

```text
MODULE 34 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

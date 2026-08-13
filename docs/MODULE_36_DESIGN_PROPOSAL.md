# SIRAGUGAL FILM STUDIO — MODULE 36 DESIGN PROPOSAL
## PROJECT BACKUP, VERSION CONTROL SNAPSHOT & AUTO-SAVE ENGINE (`sira-backup-engine`)

**Authoritative Target Repository**: `~/Siragugal` (macOS Apple Silicon Target)  
**GitHub Repository**: `https://github.com/dcgravity99/SFS`  
**Baseline Commit**: `36f577f`  
**Architecture Certificate**: `CERT-SFS-MASTER-60-2026`  
**Target Package**: `packages/sira-backup-engine`  
**Target Module File**: `packages/sira-backup-engine/src/auto_save.rs`  
**Chief Software Architect**: AG (Permanent Chief Software Architect)  
**Design Status**: 🟢 **DESIGN PROPOSED — AWAITING PROJECT OWNER APPROVAL (0 CODE IMPLEMENTED)**  

---

## 1. Executive Summary
Module 36 introduces periodic auto-saving, non-destructive version snapshots, and disaster recovery restore checkpoints to `packages/sira-backup-engine`.

## 2. Authoritative Package Boundary
- **Package**: `packages/sira-backup-engine`
- **Target File**: `packages/sira-backup-engine/src/auto_save.rs`

## 3. Existing Dependencies & Integration
- Consumes: `sfsp_engine` (Module 29), `sira_types`.

## 4. Responsibilities & Non-Responsibilities
- **Responsibilities**: Scheduled background snapshotting, differential history indexing, disaster recovery restore.
- **Non-Responsibilities**: Final export packaging (Module 30).

## 5. Data Contracts
```rust
use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupSnapshotSpec {
    pub snapshot_id: String,
    pub project_path: String,
    pub snapshot_reason: String, // "AutoSave", "PreRender", "Manual"
    pub created_at_utc: String,
}

pub struct AutoSaveBackupEngine;
```

## 6. Public APIs
```rust
impl AutoSaveBackupEngine {
    pub fn new() -> Self;
    pub fn trigger_snapshot(&self, spec: &BackupSnapshotSpec) -> SiraResult<String>;
    pub fn list_snapshots(&self, project_path: &str) -> SiraResult<Vec<BackupSnapshotSpec>>;
}
```

## 7. Future Implementation Plan
- `[NEW] packages/sira-backup-engine/src/auto_save.rs`
- `[MODIFY] packages/sira-backup-engine/src/lib.rs`

---

```text
MODULE 36 DESIGN STATUS = PROPOSED ONLY
SOURCE CODE MODIFICATIONS = NONE
COMMITS = NONE
PUSHES = NONE
GOVERNANCE STOP = ACTIVE
```

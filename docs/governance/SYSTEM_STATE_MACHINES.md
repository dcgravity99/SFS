# SYSTEM STATE MACHINES
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED ARCHITECTURE STATE MACHINE SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Project Lifecycle State Machine (`sfsp_engine`)

```mermaid
stateDiagram-v2
    [*] --> Created: SfspProject::create()
    Created --> Opened: SfspProject::open()
    Opened --> ActiveEditing: User Interaction
    ActiveEditing --> StagingSave: Save Requested
    StagingSave --> Saved: Atomic File Replacement Complete
    Saved --> ActiveEditing: Continued Editing
    ActiveEditing --> Locked: Process Active (project.lock)
    Locked --> Closed: SfspProject Close
    Closed --> [*]
```

---

## 2. Workflow Lifecycle State Machine (`workflow_engine`)

```mermaid
stateDiagram-v2
    [*] --> Idle: Workflow Constructed
    Idle --> Validating: validate_dag() Triggered
    Validating --> Validated: No Cycles (SIRA-5012) & Ports Compatible
    Validating --> Invalid: Cycle Detected or Port Mismatch
    Invalid --> Idle: User Corrects Graph
    Validated --> Executing: Execution Started
    Executing --> Checkpointed: Node Checkpoint Saved
    Checkpointed --> Executing: Next Node Processing
    Executing --> Completed: All Nodes Finished
    Executing --> Cancelled: Cancellation Token Cancelled
    Executing --> Failed: Execution Error
    Completed --> [*]
    Cancelled --> [*]
    Failed --> [*]
```

---

## 3. Plugin Lifecycle State Machine (`plugin_runtime`)

```mermaid
stateDiagram-v2
    [*] --> Installed: Plugin Package Unpacked
    Installed --> Validated: Manifest & Ed25519 Signature Verified
    Validated --> Enabled: User / System Enable
    Enabled --> Initialized: WASM Engine Loaded
    Initialized --> Started: Host APIs Bound
    Started --> ActiveRunning: Executing Workflows
    ActiveRunning --> Suspended: Memory Quota Exceeded
    Suspended --> Resumed: Resources Freed
    ActiveRunning --> Stopped: Plugin Stopped
    Stopped --> Disabled: User Disable
    Disabled --> Uninstalled: Plugin Removed
    Uninstalled --> [*]
```

---

## 4. Asset Lifecycle State Machine (`asset_db`)

```mermaid
stateDiagram-v2
    [*] --> Draft: Asset Created
    Draft --> Generated: AI Inference Complete
    Draft --> Imported: File Imported
    Generated --> Edited: User Modifications
    Imported --> Edited: User Modifications
    Edited --> Approved: Director Review Passed
    Approved --> Published: Final Film Export
    Published --> Archived: Long-term Storage
    Archived --> SoftDeleted: User Deletes Asset
    SoftDeleted --> [*]: Purged from Database
```

---

## 5. Render Job Lifecycle State Machine (`resource_manager` & `sira_hal`)

```mermaid
stateDiagram-v2
    [*] --> Queued: Render Job Submitted
    Queued --> ResourceAllocated: VRAM Lease Acquired
    ResourceAllocated --> Rendering: Compute Stream Dispatched
    Rendering --> FrameCompleted: Frame Synthesized
    FrameCompleted --> Rendering: More Frames Remaining
    FrameCompleted --> Finished: Final Frame Composited
    Rendering --> Failed: VRAM OOM / Driver Reset
    Queued --> Cancelled: User Cancel
    Finished --> [*]
    Failed --> [*]
    Cancelled --> [*]
```

---

## 6. AI Job Lifecycle State Machine (`sira_core`)

```mermaid
stateDiagram-v2
    [*] --> Pending: Job Enqueued
    Pending --> Scheduled: Priority Scheduler Selected
    Scheduled --> Running: Dispatched to AI Provider
    Running --> ProgressUpdate: Streaming Output Chunks
    ProgressUpdate --> Running: Continued Processing
    Running --> Completed: Output Returned & Usage Recorded
    Running --> Failed: Provider Error / Timeout
    Running --> Cancelled: User Cancelled Token
    Completed --> [*]
    Failed --> [*]
    Cancelled --> [*]
```

---

## 7. Model Lifecycle State Machine (`sira_ai_provider`)

```mermaid
stateDiagram-v2
    [*] --> Discovered: Weight File Present
    Discovered --> Verifying: SHA-256 Checksum Verification
    Verifying --> Verified: Checksum Match (SIRA-3008 Validated)
    Verifying --> Corrupted: Checksum Mismatch (SIRA-3008)
    Verified --> Idle: Registered in ModelRegistry
    Idle --> LoadingVram: Model Requested
    LoadingVram --> ResidentVram: Pinned in VRAM (ModelResidencyManager)
    ResidentVram --> Evicting: Memory Pressure Critical
    Evicting --> Idle: Evicted to SSD Cache
    Corrupted --> [*]
```

---

## 8. Cache Lifecycle State Machine (`cache_manager`)

```mermaid
stateDiagram-v2
    [*] --> Created: Artifact Generated
    Created --> Tier1RAM: Cached in RAM
    Tier1RAM --> SpilledSSD: High Memory Pressure (RAM -> SSD Spilling)
    SpilledSSD --> Tier1RAM: Read Hit
    SpilledSSD --> Evicted: Storage Quota Exceeded (LRU Eviction)
    Tier1RAM --> Purged: Expired TTL
    Evicted --> [*]
    Purged --> [*]
```

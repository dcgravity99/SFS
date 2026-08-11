# MODULE 10 DESIGN SPECIFICATION: SIRA AI CORE RUNTIME
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 10 establishes the core AI runtime engine (`sira-core`) for **Siragugal Film Studio**. It implements capability-driven task dispatching, unified job models, multi-tier scheduler policies, workflow checkpointing, comprehensive cancellation frameworks, runtime event buses, resource contracts, fault domain sub-process isolation per ADR-0002, and standardized telemetry without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Capability-Driven Dispatching**: Jobs request capabilities (`AICapability::TextGeneration`, `VideoGeneration`, `AudioGeneration`, `SpeechToText`, etc.), delegating provider selection to Module 11.
2. **Unified SiraJob Model**: Standardized job struct containing `job_id`, `parent_job_id`, `workflow_id`, `capability`, `priority`, `state`, `progress`, `retry_count`, `cancellation_token`, `resource_contract`, `estimated_cost`, and `estimated_duration`.
3. **Multi-Tier Scheduler Policies**:
   - `Interactive`: Real-time user input (lowest latency priority).
   - `Background`: Timeline preview renders.
   - `Batch`: Final export renders.
   - `RealTime`: Low-latency audio/video streaming.
   - `LowPower`: Energy-saving laptop execution.
4. **Workflow Checkpointing Engine**: Saves execution checkpoints enabling Pause, Resume, Restart, Rollback, and crash recovery.
5. **Comprehensive Cancellation Framework**: Graceful cancellation, forced SIGKILL termination, timeout enforcement, cascading dependency cancellation, and user cancellation tokens.
6. **Runtime Event Bus (`SiraEventBus`)**: Publishes standardized events (`JobStarted`, `JobProgress`, `JobCompleted`, `JobFailed`, `EngineStarted`, `EngineStopped`, `ResourceAllocated`, `ProviderChanged`).
7. **Resource Contract Enforcement**: Enforces explicit VRAM, RAM, CPU, GPU, disk, and network bandwidth contracts per job.
8. **Isolated Fault Domains**: Enforces sub-process isolation for all 11 sub-engines per ADR-0002.
9. **Runtime Telemetry Sampler**: Tracks active jobs, queue depth, throughput, resource usage, and retry rates.

---

## 3. Unified Job Model Schema

```json
{
  "job_id": "job-018d9b12-42a1-7910-8b14-c12e5fa90123",
  "parent_job_id": null,
  "workflow_id": "wf-1042",
  "capability": "VideoGeneration",
  "priority_policy": "Interactive",
  "state": "RUNNING",
  "progress": 0.45,
  "retry_count": 0,
  "resource_contract": {
    "vram_mb": 12288,
    "ram_mb": 16384,
    "cpu_cores": 4,
    "gpu_count": 1
  },
  "estimated_cost_usd": 0.0,
  "estimated_duration_sec": 12.5
}
```

---

## 4. File Blueprint

Module 10 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-core/                  # Rust SIRA AI Core Runtime crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & SiraCoreRuntime API
            ├── capabilities.rs     # AICapability enum & registry
            ├── job.rs              # Unified SiraJob model & ResourceContract
            ├── scheduler.rs        # Multi-tier priority scheduler (Interactive, Batch, etc.)
            ├── checkpoint.rs       # Workflow state checkpointing & recovery engine
            ├── cancellation.rs     # Cancellation token & timeout framework
            ├── event_bus.rs        # Runtime SiraEventBus dispatcher
            ├── manager.rs          # Isolated fault-domain process supervisor
            └── telemetry.rs        # Core runtime telemetry sampler
```

---

## 5. Acceptance Criteria

Module 10 is accepted when:
1. `packages/sira-core` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Capability-driven job scheduling executes jobs across priority policies correctly.
3. Simulating sub-engine process failure recovers state gracefully without losing unrelated jobs or workflow state.
4. Workflow checkpointing and cancellation tokens pass 100% of integration tests.
5. Zero application or creative feature code is present.

# ARCHITECTURE SEQUENCE DIAGRAMS
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED ARCHITECTURE REFERENCE DIAGRAMS  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Project Creation

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant UI as Presentation Layer
    participant SFSP as sfsp_engine
    participant DB as asset_db (SQLite)
    participant Lock as ProjectLock

    User->>UI: Click "New Project" ("Wings of Destiny")
    UI->>SFSP: SfspProject::create("Wings of Destiny", path)
    SFSP->>SFSP: Create directory & sub-directories (assets/, graph/, models/)
    SFSP->>Lock: ProjectLock::acquire(project_dir)
    Lock-->>SFSP: Lock Acquired
    SFSP->>DB: Initialize project.db (WAL mode) & assets tables
    SFSP->>SFSP: Write manifest.json
    SFSP-->>UI: Return SfspProject Handle
    UI-->>User: Display Empty Project Workspace
```

---

## 2. Project Open

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant UI as Presentation Layer
    participant SFSP as sfsp_engine
    participant Lock as ProjectLock
    participant Integrity as SHA256 Integrity
    participant DB as asset_db

    User->>UI: Select Project (.sfsp)
    UI->>SFSP: SfspProject::open(project_path)
    SFSP->>Lock: ProjectLock::acquire(project_path)
    Lock-->>SFSP: Lock Acquired
    SFSP->>Integrity: compute_sha256(manifest.json & project.db)
    Integrity-->>SFSP: Integrity Validated
    SFSP->>DB: Open project.db & execute FTS5 index verify
    SFSP-->>UI: Return Active SfspProject
    UI-->>User: Render Project Dashboard & Asset Grid
```

---

## 3. Voice → Film Workflow Execution

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant UI as Presentation Layer
    participant Core as sira_core Runtime
    participant Provider as sira_ai_provider
    participant HAL as sira_hal
    participant Workflow as workflow_engine

    User->>UI: Provide Voice Audio Track
    UI->>Core: Dispatch Voice-to-Film Job
    Core->>Provider: Request SpeechToText & AudioGeneration
    Provider->>Provider: Resolve Local/Cloud Provider (Offline-First)
    Provider->>HAL: Allocate VRAM Lease & Dispatch Compute Stream
    HAL-->>Provider: Compute Stream Completed
    Provider-->>Core: Timed Lip-Sync & Audio PCM Chunks
    Core->>Workflow: Construct Generative Scene DAG
    Workflow->>Workflow: Execute DAG Nodes (Topological Order)
    Workflow-->>UI: Stream Preview Frame Buffer
```

---

## 4. Text → Film Workflow Execution

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant UI as Presentation Layer
    participant Core as sira_core
    participant Router as ProviderRouter
    participant HAL as sira_hal

    User->>UI: Input Prompt ("Cinematic dragon over mountains")
    UI->>Core: Submit Text-to-Film Job
    Core->>Router: Select Best Provider (VideoGeneration)
    Router-->>Core: Selected Local Diffusers Provider
    Core->>HAL: Reserve DeviceVram & Dispatch Inference Kernel
    HAL-->>Core: Return Synthesized Frame Buffer (Zero-Copy)
    Core-->>UI: Update Timeline Viewport
```

---

## 5. Script → Film Workflow Execution

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant Story as Story Engine (Module 17)
    participant Director as Director Engine (Module 21)
    participant Scene as Scene Engine (Module 20)
    participant Render as Render Engine (Module 25)

    User->>Story: Upload Screenplay (.fountain)
    Story->>Story: Parse Scene Beats & Character Dialogue
    Story->>Director: Forward Beat Sheet Graph
    Director->>Director: Synthesize Shot List & Camera Framing
    Director->>Scene: Generate 3D Spatial Layout Matrix
    Scene->>Render: Dispatch Shot Composition Render DAG
    Render-->>User: Display Assembled Rough Cut Film
```

---

## 6. Workflow DAG Execution Pipeline

```mermaid
sequenceDiagram
    autonumber
    participant Engine as workflow_engine
    participant DAG as DagValidator
    participant Cache as CacheManager
    participant Core as sira_core

    Engine->>DAG: DagValidator::validate_and_toposort(nodes, edges)
    DAG-->>Engine: Topological Node Order (No Cycles - SIRA-5012)
    loop Every Node in Order
        Engine->>Cache: compute_node_input_hash(node, inputs)
        alt Cache Hit
            Cache-->>Engine: Return Cached Artifact (Skip Re-run)
        else Cache Miss
            Engine->>Core: SiraCoreRuntime::dispatch_job(node_job)
            Core-->>Engine: Return Job Execution Result
            Engine->>Cache: Store Output Artifact
        end
    end
```

---

## 7. Plugin Execution Sandbox Flow

```mermaid
sequenceDiagram
    autonumber
    participant Workflow as workflow_engine
    participant Runtime as plugin_runtime
    participant Validator as PermissionValidator
    participant Sandbox as Wasmtime WASI Sandbox

    Workflow->>Runtime: execute_plugin_node(plugin_id, inputs)
    Runtime->>Validator: verify_permission(plugin_permissions, required)
    alt Permission Granted
        Validator-->>Runtime: Permission Verified
        Runtime->>Sandbox: Execute WASM Module Function
        Sandbox-->>Runtime: Return Execution Result JSON
        Runtime-->>Workflow: Return Node Output
    else Permission Denied
        Validator-->>Runtime: Error SIRA-6004 (Permission Denied)
        Runtime-->>Workflow: Return SIRA-6004 Error Result
    end
```

---

## 8. AI Provider Request Flow

```mermaid
sequenceDiagram
    autonumber
    participant Core as sira_core
    participant Router as ProviderRouter
    participant Provider as AiProvider (Mock / Cloud / Local)
    participant Security as SecurityManager

    Core->>Router: resolve_best_provider(AICapability::TextGen)
    Router->>Router: Evaluate Offline-First Chain (Local -> Enterprise -> Cloud)
    Router-->>Core: Selected Provider Handle
    Core->>Security: get_api_key_from_keychain(provider_id)
    Security-->>Core: Return Protected Credentials
    Core->>Provider: execute(AIRequest)
    Provider-->>Core: Return AIResponse (Text / Media URI / Usage)
```

---

## 9. Render Pipeline

```mermaid
sequenceDiagram
    autonumber
    participant Scheduler as Render Scheduler
    participant Resource as resource_manager
    participant HAL as sira_hal
    participant Cache as cache_manager

    Scheduler->>Resource: ResourceReservation::reserve(VRAM, RAM)
    Resource-->>Scheduler: Active ResourceLease Token
    Scheduler->>HAL: Dispatch Compute Stream (Metal / CUDA / CPU)
    HAL-->>Scheduler: Render Output Frame Buffer
    Scheduler->>Cache: Store Frame in Tier 2 NVMe SSD Cache
    Scheduler->>Resource: ResourceLease::release(lease_token)
```

---

## 10. Project Save

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant UI as Presentation Layer
    participant SFSP as sfsp_engine
    participant DB as asset_db

    User->>UI: Save Project (Ctrl+S / Cmd+S)
    UI->>SFSP: SfspProject::save()
    SFSP->>DB: Commit SQLite WAL Transaction
    SFSP->>SFSP: Stage project files in .sfsp.tmp/
    SFSP->>SFSP: Sync files & Atomic Directory Rename
    SFSP-->>UI: Save Confirmed
```

---

## 11. Universal Undo / Redo Flow (ADR-0004)

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant UI as Presentation Layer
    participant Exp as Experience Layer
    participant DB as project.db (SQLite)

    User->>UI: Trigger Action (e.g. Modify Camera Position)
    UI->>Exp: Execute Command
    Exp->>DB: Record Inverse Undo SQL & Forward Redo SQL
    Exp-->>UI: Update Viewport
    User->>UI: Press Undo (Ctrl+Z / Cmd+Z)
    UI->>Exp: Undo Command
    Exp->>DB: Execute Inverse Undo SQL Statement
    Exp-->>UI: Restore Previous Viewport State
```

---

## 12. Crash Recovery Flow

```mermaid
sequenceDiagram
    autonumber
    participant Startup as Studio Bootstrapper
    participant Lock as ProjectLock
    participant Recovery as CacheRecoveryEngine
    participant Core as sira_core

    Startup->>Lock: Check for Stale project.lock File
    alt Stale Lock Detected (>10m Inactive PID)
        Lock->>Lock: Execute Safe Lock Recovery & Log Diagnostic
    end
    Startup->>Recovery: perform_startup_recovery()
    Recovery->>Recovery: Repair SQLite cache.db index & purge orphaned partial files
    Startup->>Core: Start SiraCoreRuntime supervisor
```

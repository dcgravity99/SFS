# MODULE 14 DESIGN SPECIFICATION: RESOURCE MANAGER
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 14 establishes the centralized system Resource Manager (`resource-manager`) for **Siragugal Film Studio**. It implements unified atomic resource reservations, priority-based scheduling, lease-based allocation lifecycles, predictive resource estimation, multi-GPU affinity pools, live telemetry streaming, configurable resource policies, integration stress testing, comprehensive failure recovery, and future-ready distributed resource lease abstractions specified in [docs/architecture/resource_and_cache_architecture.md](file:///D:/SiragugalFilmStudio/docs/architecture/resource_and_cache_architecture.md) without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Unified Resource Reservation API**: Single atomic request reserving RAM, VRAM, CPU cores, GPU devices, and disk I/O bandwidth (`ResourceReservation`).
2. **Priority-Based Scheduling**: Prioritizes `Interactive` UI tasks over `Background` previews and `Batch` render jobs.
3. **Lease-Based Allocation Lifecycle**: Manages resource leases (`ResourceLease`) with explicit `acquire()`, `renew()`, `release()`, and `auto_expire()` heartbeat lifecycles.
4. **Predictive Resource Estimation**: Estimates required VRAM and RAM footprint using historical model heuristics before job dispatch.
5. **Multi-GPU Affinity Pools**: Manages multi-GPU device allocation, P2P memory transfers, and GPU device affinity masks.
6. **Live Resource Telemetry**: Streams real-time VRAM allocation, RAM usage, CPU %, GPU %, thermal state, and battery status to `sira-diagnostics`.
7. **Configurable Resource Policies**: Supports user-customizable VRAM allocation caps, memory pressure thresholds, and battery-saver throttling modes.
8. **Comprehensive Failure Recovery & LRU Eviction**: Triggers automated LRU model weight eviction under `Critical` memory pressure, gracefully degrading tasks to CPU RAM.
9. **Distributed Resource Abstraction**: Exposes lease interfaces compatible with local, LAN render node, and cloud resource clusters.

---

## 3. Unified Resource Reservation Schema

```json
{
  "reservation_id": "res-018d9b12-42a1-7910-8b14-c12e5fa90123",
  "client_id": "sira-core-job-4012",
  "priority_policy": "Interactive",
  "requested_resources": {
    "vram_mb": 8192,
    "ram_mb": 16384,
    "cpu_cores": 4,
    "gpu_count": 1,
    "disk_io_mbps": 100
  },
  "ttl_seconds": 60,
  "lease_state": "ACTIVE"
}
```

---

## 4. File Blueprint

Module 14 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── resource-manager/           # Rust Resource Manager crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & ResourceManager API
            ├── reservation.rs      # Unified ResourceReservation data structures
            ├── lease.rs            # ResourceLease lifecycle (acquire/renew/release/expire)
            ├── vram_pool.rs        # VRAM allocation pool & limits enforcer
            ├── ram_pool.rs         # System RAM monitor & memory pressure calculator
            ├── thread_pool.rs      # CPU core & thread pool allocator
            ├── predictive.rs       # Predictive VRAM/RAM footprint estimator
            ├── gpu_pool.rs         # Multi-GPU affinity pool & P2P manager
            ├── telemetry.rs        # Live resource telemetry sampler
            ├── policies.rs         # Configurable resource allocation policies
            └── eviction.rs         # LRU model weight eviction engine & crash recovery
```

---

## 5. Acceptance Criteria

Module 14 is accepted when:
1. `packages/resource-manager` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Unified atomic resource reservations and lease lifecycles pass 100% of integration tests.
3. Under simulated `Critical` memory pressure, LRU eviction successfully frees VRAM/RAM without crashing active jobs.
4. Stress tests with 100 concurrent reservation requests execute without deadlock or resource leaks.
5. Zero application or creative feature code is present.

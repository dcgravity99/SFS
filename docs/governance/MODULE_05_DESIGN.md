# MODULE 05 DESIGN SPECIFICATION: LOGGING & DIAGNOSTICS
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 05 establishes the enterprise logging, diagnostics, performance instrumentation, crash classification, and support bundling engine (`sira-diagnostics`) for **Siragugal Film Studio**. It implements structured JSON logging, distributed OpenTelemetry-compatible tracing contexts, automatic sensitive data redaction, non-blocking log rotation, compression, cleanup, and diagnostic bundle generation without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **OpenTelemetry-Compatible Distributed Trace Context**: Pass `trace_id`, `span_id`, `parent_span_id`, `workflow_id`, and `project_id` across async task boundaries and IPC calls.
2. **Structured Diagnostic Event Categories**: Categorize logs under explicit domains: `SYSTEM`, `HAL`, `AI_MODEL`, `RENDER`, `PLUGIN`, `WORKFLOW`, and `STORAGE`.
3. **Automatic Sensitive-Data Redaction Engine**: Automatically sanitize log messages and context maps using regex rules, stripping API keys (`sk-...`), tokens, passwords, and user script contents.
4. **Diagnostic Support Bundle Generator (`sira-diagnostics bundle`)**: Package system logs, config diagnostics, HAL device reports, and crash traces into a compressed zip file (`sira-support-bundle-TIMESTAMP.zip`).
5. **Performance Instrumentation**: Measure task execution time (ms), RSS memory (MB), VRAM usage (MB), IPC latency (ms), and disk I/O bandwidth (MB/s).
6. **Log Retention, Compression & Cleanup Policy**: Rotate logs at 10MB; compress logs older than 7 days (Gzip); auto-purge logs older than 30 days or when total log storage exceeds 100MB quota.
7. **Crash Classification Framework**: Categorize crashes into `FATAL_PANIC`, `OOM_CRASH`, `HAL_DRIVER_RESET`, and `PLUGIN_SEGFAULT`.
8. **Lightweight Health Reporting Interface**: Expose `HealthReport` struct returning overall system health (`Healthy`, `Degraded`, `Unhealthy`).

---

## 3. OpenTelemetry-Compatible JSON Log Schema

```json
{
  "timestamp": "2026-08-03T10:05:00.123Z",
  "level": "INFO",
  "category": "AI_MODEL",
  "subsystem": "sira-core",
  "message": "Model weights loaded successfully",
  "trace_context": {
    "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736",
    "span_id": "00f067aa0ba902b7",
    "parent_span_id": "5fa42a10c8b9d012",
    "workflow_id": "wf-89102",
    "project_id": "proj-1042"
  },
  "performance": {
    "duration_ms": 142.5,
    "memory_rss_mb": 420,
    "vram_mb": 8192
  },
  "context": {
    "model_id": "sira-diffusers-sdxl-v1"
  }
}
```

---

## 4. File Blueprint

Module 05 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-diagnostics/           # Rust diagnostics engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & macros
            ├── logger.rs           # Tracing subscriber & OpenTelemetry JSON formatter
            ├── context.rs          # Trace ID, Span ID, Workflow ID context tracker
            ├── redact.rs           # Sensitive-data redaction engine (Regex patterns)
            ├── rotation.rs         # Log file rotator, Gzip compressor & cleanup policy
            ├── panic_handler.rs    # Crash classification framework & panic report writer
            ├── bundle.rs           # Support diagnostic bundle packager
            └── health.rs           # System health monitoring interface
```

---

## 5. Acceptance Criteria

Module 05 is accepted when:
1. `packages/sira-diagnostics` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Structured JSON logs pass schema validation and include OpenTelemetry-compatible trace contexts.
3. Redaction engine unit tests verify API keys (`sk-...`) and tokens are 100% redacted before writing to log files.
4. Support bundle generator (`sira-diagnostics bundle`) produces valid diagnostic zip archives.
5. Zero application or creative feature code is present.

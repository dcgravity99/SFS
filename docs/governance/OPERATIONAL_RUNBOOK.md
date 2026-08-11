# OPERATIONAL RUNBOOK & TROUBLESHOOTING GUIDE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED OPERATIONAL RUNBOOK  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Executive Summary

This Operational Runbook documents automated and manual recovery procedures for system administrators, developers, and users of **Siragugal Film Studio**.

---

## 2. Standard Operating Procedures (SOP)

### SOP-01: Project Recovery (`.sfsp`)
- **Symptom**: Stale `project.lock` or corrupted `manifest.json`.
- **Procedure**:
  1. Inspect `manifest.json.v1.bak`. If present, restore backup manifest.
  2. If `project.lock` exists with inactive PID (>10 minutes), execute `sfsp_engine::ProjectLock::release()`.
  3. Verify `project.db` integrity using `sqlite3 project.db "PRAGMA quick_check;"`.

### SOP-02: Cache Rebuild (`cache.db`)
- **Symptom**: Disk space full or corrupted `cache.db` metadata index.
- **Procedure**:
  1. Execute `CacheMaintenanceService::run_maintenance_pass()`.
  2. To force complete rebuild, delete `.sfsp/cache/` directory.
  3. Execute `CacheRecoveryEngine::perform_startup_recovery()` to re-index.

### SOP-03: Plugin Recovery & Sandbox Reset
- **Symptom**: Plugin crashes with WASM memory trap or permission error `SIRA-6004`.
- **Procedure**:
  1. `plugin_runtime` automatically transitions plugin state to `Disabled`.
  2. Inspect structured crash log in `logs/crashes/plugin_<id>.json`.
  3. Re-enable plugin via settings or update to fixed WASM module release.

### SOP-04: Model Installation & Checksum Recovery
- **Symptom**: Error `SIRA-3008: MODEL_CHECKSUM_VERIFICATION_FAILED`.
- **Procedure**:
  1. Verify GGUF / Safetensors file size.
  2. Re-download model weights over TLS 1.3 HTTPS stream.
  3. Run `ModelRegistry::verify_weights_checksum(path, sha256)` to confirm SHA-256 integrity.

### SOP-05: SQLite Database Repair (`project.db`)
- **Symptom**: `SIRA-4002: SFSP_MANIFEST_CORRUPTED` or DB read failure.
- **Procedure**:
  1. Export DB to SQL dump: `sqlite3 project.db ".dump" > dump.sql`.
  2. Re-import into fresh database: `sqlite3 fixed_project.db < dump.sql`.
  3. Replace `project.db` with `fixed_project.db`.

### SOP-06: Crash Diagnostics & Support Bundle Generation
- **Symptom**: System panic or unhandled exception.
- **Procedure**:
  1. `sira_diagnostics::register_panic_hook()` writes panic report to `logs/crashes/panic_<timestamp>.json`.
  2. Run `sira_diagnostics::generate_support_bundle()` to output `sira-support-bundle-TIMESTAMP.zip`.

### SOP-07: Workflow Execution Resume
- **Symptom**: Render job interrupted by power loss.
- **Procedure**:
  1. On studio launch, `workflow_engine` loads `WorkflowExecutionCheckpoint`.
  2. Compare node input hashes (`node_hash`) against completed node outputs in cache.
  3. Resume execution seamlessly from last un-rendered DAG node.

### SOP-08: GPU Reset & Driver Timeout Recovery
- **Symptom**: GPU driver reset or TDR timeout on Windows.
- **Procedure**:
  1. `sira_hal` catches GPU device lost signal.
  2. Evict VRAM allocations and re-initialize `DeviceCapabilityRegistry`.
  3. Automatically fail over to secondary GPU or CPU compute fallback.

### SOP-09: VRAM Out-of-Memory (OOM) Recovery (`SIRA-2015`)
- **Symptom**: Error `SIRA-2015: CUDA_VRAM_ALLOCATION_OOM`.
- **Procedure**:
  1. `resource_manager` receives VRAM allocation failure notification.
  2. Execute `LruEvictionEngine::trigger_emergency_eviction()`.
  3. Unload idle LoRA weights and spill RAM cache to NVMe SSD disk cache.
  4. Retry VRAM reservation lease request.

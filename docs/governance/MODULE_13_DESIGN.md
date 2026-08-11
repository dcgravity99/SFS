# MODULE 13 DESIGN SPECIFICATION: PLUGIN RUNTIME
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 13 establishes the sandboxed WebAssembly (WASM) Plugin Runtime (`plugin-runtime`) for **Siragugal Film Studio**. It implements the versioned Plugin SDK, expanded manifest specifications, 11-step plugin lifecycles, capability registries, fine-grained 10-tier permission models (`SIRA-6004`), resource quota enforcers, dependency resolution engines, Ed25519 digital signature verifiers, stable host APIs, plugin event buses, and complete crash isolation without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Versioned Plugin SDK Specification**: Explicit `PluginSdkVersion`, API compatibility policy, and deprecation schedules.
2. **Standardized 11-Step Plugin Lifecycle**: `Install` → `Validate` → `Enable` → `Initialize` → `Start` → `Suspend` → `Resume` → `Stop` → `Disable` → `Update` → `Uninstall`.
3. **Plugin Capability Registration**: Plugins register capabilities (`WorkflowNode`, `AiProvider`, `MediaImporter`, `MediaExporter`, `AssetType`, `UiExtension`, `TimelineEffect`) through `PluginCapabilityRegistry`.
4. **10-Tier Fine-Grained Permission Model**: Enforces least privilege (`sira.permission.PROJECT_READ`, `ASSET_WRITE`, `FILESYSTEM_READ`, `NETWORK_FETCH`, `GPU_ACCESS`, etc.). Emits structured error `SIRA-6004: PLUGIN_PERMISSION_DENIED` on violations.
5. **Configurable Resource Quotas**: Enforces strict RAM (MB), VRAM (MB), CPU %, GPU %, max threads, execution time limits (sec), disk (MB), and network limits per plugin instance.
6. **Dependency Resolution Engine**: Resolves SemVer constraints (`^1.2.0`), optional dependencies, conflict detection, and circular dependency checks.
7. **Ed25519 Digital Signature Verification**: Verifies publisher identity, signature validity, SHA-256 checksums, and assigns trust levels (`Core`, `Verified`, `Community`, `Untrusted`).
8. **Stable Versioned Host APIs**: Groups host imports into 10 isolated modules (Logging, Asset DB, Workflow, AI Provider, Timeline, Render, Config, Settings, Diagnostics, Project System).
9. **Plugin Event Bus (`PluginEventBus`)**: Publishes real-time events (`ProjectOpen`, `ProjectSave`, `AssetAdded`, `WorkflowStarted`, `RenderFinished`, `SettingsChanged`) to plugins without polling.
10. **Crash Isolation & Recovery**: WASM traps catch memory panics and unauthorized I/O cleanly, automatically disabling the failing plugin without destabilizing Studio.

---

## 3. Expanded Plugin Manifest Schema (`plugin.json`)

```json
{
  "sdk_version": "1.0.0",
  "plugin_id": "plugin-studio-lut-effects",
  "name": "Cinematic LUT Color Effects",
  "version": "1.2.0",
  "publisher": {
    "name": "Studio FX Corp",
    "publisher_id": "pub-98102",
    "trust_level": "Verified"
  },
  "dependencies": {
    "@sira/core-types": "^1.0.0"
  },
  "capabilities": [
    "WorkflowNode",
    "TimelineEffect"
  ],
  "permissions": [
    "sira.permission.READ_ASSET",
    "sira.permission.GPU_ACCESS"
  ],
  "resource_quotas": {
    "max_ram_mb": 1024,
    "max_vram_mb": 2048,
    "max_execution_time_sec": 30.0
  },
  "signature": {
    "algorithm": "Ed25519",
    "public_key": "31e35cde5e003f1d...",
    "signature_hex": "4bf92f3577b34da..."
  }
}
```

---

## 4. File Blueprint

Module 13 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── plugin-runtime/             # Rust WASM Plugin Runtime crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & PluginRuntime API
            ├── sdk.rs              # PluginSdkVersion & API compatibility policy
            ├── manifest.rs         # plugin.json manifest parser & publisher metadata
            ├── lifecycle.rs        # 11-step plugin lifecycle state machine
            ├── capabilities.rs     # PluginCapabilityRegistry & registration hooks
            ├── permissions.rs      # 10-tier permission boundary validator (SIRA-6004)
            ├── quotas.rs           # Resource quota enforcer (RAM, VRAM, CPU)
            ├── dependencies.rs     # SemVer dependency resolution engine
            ├── signing.rs          # Ed25519 digital signature & trust verifier
            ├── host_apis.rs        # 10 grouped versioned Host API bindings
            ├── event_bus.rs        # PluginEventBus event dispatcher
            └── sandbox.rs          # Wasmtime WASI sandbox & crash trap handler
```

---

## 5. Acceptance Criteria

Module 13 is accepted when:
1. `packages/plugin-runtime` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Any third-party plugin implementing the public Plugin SDK and permission model can be installed, executed, updated, and removed with zero modifications to SIRA Core, Workflow Engine, HAL, or other core modules.
3. Permission violations emit structured error `SIRA-6004: PLUGIN_PERMISSION_DENIED`.
4. WASM sandboxes trap memory panics cleanly without crashing Studio.
5. Zero application or creative feature code is present.

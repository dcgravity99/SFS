# MODULE 29 DESIGN SPECIFICATION: EXTENSION & PLUGIN ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 29 establishes the **Extension & Plugin Engine** (`sira-engine-plugin`) for **Siragugal Film Studio**. It implements WebAssembly / WASI plugin host sandboxing (`plugin_runtime`), 10-tier permission enforcement (`SIRA-6004`), Host API dispatching across all 12 SIRA sub-engines, plugin lifecycle management (`Installed` → `Uninstalled`), and Ed25519 digital signature verification specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or application feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Wasmtime WASI Plugin Sandbox Host**: Execute third-party plugins in memory-isolated WASI sandboxes with strict CPU / memory resource quotas (`plugin_runtime`).
2. **10-Tier Permission Checker**: Enforce granular permission scopes (`ReadProject`, `WriteAsset`, `ExecuteAI`, `NetworkAccess`, `FileSystemAccess`).
3. **Sub-Engine Host API Dispatcher**: Expose controlled Host APIs allowing plugins to register custom story beats, character attributes, or workflow nodes without breaking system stability.
4. **Plugin Lifecycle & Signing Verifier**: Verify Ed25519 signatures and SemVer compatibility before loading plugins.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 28 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_hal`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `plugin_runtime`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`, `sira_engine_audio`, `sira_engine_timeline`, `sira_engine_render`, `sira_engine_asset`, `sira_engine_workflow`, `sira_engine_packaging`, `resource_manager`, `cache_manager`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 28](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_28_COMPLETION.md).

---

## 4. Public Interfaces

Module 29 exposes public extension & plugin engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_plugin)
pub struct ExtensionEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub required_permissions: Vec<String>,
    pub wasm_file_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginExecutionResult {
    pub plugin_id: String,
    pub is_success: bool,
    pub output_payload_json: String,
}

impl ExtensionEngine {
    pub fn load_plugin(manifest: PluginManifest) -> SiraResult<String>;
    pub fn execute_plugin_hook(plugin_id: &str, hook_name: &str, input_json: &str) -> SiraResult<PluginExecutionResult>;
    pub fn unload_plugin(plugin_id: &str) -> SiraResult<()>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 29 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-plugin/         # Rust Extension & Plugin Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & ExtensionEngine API
            ├── host.rs             # Wasmtime WASI sandbox host manager
            ├── permissions.rs      # 10-tier permission enforcement engine
            ├── host_apis.rs        # Sub-engine Host API dispatcher
            └── lifecycle.rs        # Plugin lifecycle & SemVer resolver
```

---

## 6. Testing & Validation Strategy

1. **Plugin Loading & Sandbox Test**: Load sample WASM plugin; verify Wasmtime sandbox initializes with memory quotas.
2. **Permission Violation Test**: Invoke restricted API without granted permission; verify `SIRA-6004` permission denied error is returned.
3. **Plugin Hook Execution Test**: Execute plugin hook; verify output payload is received cleanly without host panic.

---

## 7. Acceptance Criteria

Module 29 is accepted when:
1. `packages/sira-engine-plugin` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Plugin loading, 10-tier permission enforcement, and hook execution pass 100% of unit tests.
3. Zero UI or application feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 29: Extension & Plugin Engine**.
> 2. Upon your explicit approval, I will execute Module 29 implementation (`packages/sira-engine-plugin`).

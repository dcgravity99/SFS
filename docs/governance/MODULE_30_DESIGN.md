# MODULE 30 DESIGN SPECIFICATION: STUDIO APPLICATION & DESKTOP SHELL
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 30 establishes the **Studio Application & Desktop Shell** (`sira-studio-app`) for **Siragugal Film Studio**. It completes Phase 2 by implementing the Tauri 2.0 / Rust IPC desktop shell host, multi-window workspace manager, native menu integration, presentation layer event bridge (`experience_layer`), and main application startup bootstrapper specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md).

---

## 2. Module Responsibilities & Core Features

1. **Desktop Shell Host & IPC Bridge**: Host the Tauri 2.0 native window process and translate frontend IPC messages into asynchronous Rust calls across Modules 00 through 29.
2. **Multi-Window Workspace Manager**: Manage dedicated studio windows (Screenplay Editor, 3D Scene Viewport, NLE Timeline, Node Graph, Render Monitor).
3. **Application Bootstrapper & Shutdown Orchestrator**: Initialize database connections (`project.db`, `cache.db`), hardware acceleration (`sira_hal`), background job workers, and perform graceful thread pool shutdowns.
4. **Native System Menu & Shortcut Integration**: Bind OS-level file menus, global keyboard shortcuts, and drag-and-drop file handlers.

---

## 3. Module Dependencies

- **Software Dependencies**: All Phase 1 & Phase 2 Modules 00 - 29 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_hal`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `plugin_runtime`, `resource_manager`, `cache_manager`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`, `sira_engine_audio`, `sira_engine_timeline`, `sira_engine_render`, `sira_engine_asset`, `sira_engine_workflow`, `sira_engine_packaging`, `sira_engine_plugin`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 00 - 29](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_29_COMPLETION.md).

---

## 4. Public Interfaces

Module 30 exposes public studio application interfaces across Rust:

```rust
// Rust Public Interface (sira_studio_app)
pub struct StudioApplication;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppLaunchConfig {
    pub project_file_path: Option<String>,
    pub enable_gpu_acceleration: bool,
    pub developer_mode: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowSpec {
    pub window_id: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub is_resizable: bool,
}

impl StudioApplication {
    pub fn bootstrap(config: AppLaunchConfig) -> SiraResult<StudioApplication>;
    pub fn open_window(spec: WindowSpec) -> SiraResult<()>;
    pub fn shutdown(&self) -> SiraResult<()>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 30 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-studio-app/            # Rust Studio Application & Desktop Shell crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & StudioApplication API
            ├── bootstrap.rs        # Main application startup bootstrapper
            ├── shell.rs            # Desktop shell & multi-window manager
            ├── ipc_bridge.rs       # Tauri 2.0 IPC command router
            └── menu.rs             # Native OS menu & keyboard shortcut manager
```

---

## 6. Testing & Validation Strategy

1. **Application Bootstrap Test**: Launch bootstrapper with launch config; verify all 29 underlying engine crates initialize cleanly.
2. **IPC Command Router Test**: Dispatch IPC request; verify call routes asynchronously to backend engine API and returns typed JSON response.
3. **Graceful Shutdown Test**: Trigger application shutdown; verify all worker threads, VRAM leases, and database connections release cleanly.

---

## 7. Acceptance Criteria

Module 30 is accepted when:
1. `packages/sira-studio-app` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Application bootstrap, IPC command routing, and graceful shutdown pass 100% of unit tests.
3. Module 30 successfully completes the Phase 2 Master Plan architecture baseline.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 30: Studio Application & Desktop Shell**.
> 2. Upon your explicit approval, I will execute Module 30 implementation (`packages/sira-studio-app`).

# DESIGN SPECIFICATION — PHASE E: TAURI DESKTOP INTEGRATION & RUNTIME ARCHITECTURE

**Repository**: `D:\SiragugalFilmStudio`  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR GOVERNANCE REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  
**Primary Target Platform**: macOS (`aarch64-apple-darwin` & `x86_64-apple-darwin`)  
**Development/Audit Host**: Windows 11 (`x86_64-pc-windows-msvc`)  

---

## 1. Executive Summary & Architectural Intent

Phase E establishes the native desktop host runner for **Siragugal Film Studio** using **Tauri 2.x**. This specification resolves `E-TAURI-CONFIG-MISSING` without creating duplicate binaries or violating any core product constraints:

- **Local-First & Standalone**: Zero mandatory cloud services, SaaS credentials, paid AI APIs, or telemetry uploadSDKs.
- **Single Binaries Host**: Unifies Module 30 (`packages/sira-studio-app`) as the direct Tauri 2.0 executable container, avoiding duplicate bootstrap runners.
- **Tamil-First Globalization**: `ta-IN` primary localization with `en-US` fallback across UI and window titles.
- **Cross-Platform Target Integrity**: Primary target is macOS (`aarch64-apple-darwin` / Apple Silicon and `x86_64-apple-darwin` / Intel Mac). Windows 11 host is used strictly for build verification and cross-platform compilation checks.

---

## 2. Architectural Decisions & Location Strategy

### A. Location Choice: `packages/sira-studio-app` as Native Host
Rather than creating an isolated, redundant `apps/studio-ui/src-tauri` workspace member, **`packages/sira-studio-app`** (Module 30: Studio Application & Desktop Shell) will serve as the single, authoritative Tauri 2.0 desktop application binary container.

**Architectural Rationale**:
1. **Zero Bootstrapper Duplication**: `sira_studio_app` already imports all 41 engine crates (`sira_core`, `sfsp_engine`, `asset_db`, `sira_engine_story`, `sira_engine_director`, etc.) and contains `bootstrap.rs`, `shell.rs`, `ipc_bridge.rs`, and `menu.rs`.
2. **Clean Dependency Layering**: Prevents circular dependencies between UI presentation layers and Rust engine crates.
3. **Workspace Organization**: Keeps Rust binary configuration co-located with the Rust shell host, pointing `frontendDist` directly to `../../apps/studio-ui/dist`.

---

## 3. Integration Boundary & IPC Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│                   React 19 / TypeScript Presentation                   │
│                          (apps/studio-ui)                              │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ StudioIpcService / window.__TAURI__.invoke
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     Tauri 2.0 IPC Routing Layer                        │
│             (packages/sira-studio-app/src/ipc_bridge.rs)              │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ ApplicationBootstrapper & DesktopShellManager
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    Rust Engine Infrastructure (41 Crates)               │
│  sira_core · sfsp_engine · asset_db · sira_engine_story · sira_engine_actor... │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ Local Storage & Hardware Abstraction
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│               Local Storage / GPU / Media Processing                   │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Detailed Configuration Blueprint

### A. `packages/sira-studio-app/tauri.conf.json`
```json
{
  "$schema": "https://raw.githubusercontent.com/tauri-apps/tauri/dev/crates/tauri-config-schema/schema.json",
  "productName": "Siragugal Film Studio",
  "version": "1.0.0",
  "identifier": "com.siragugal.filmstudio",
  "build": {
    "frontendDist": "../../apps/studio-ui/dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "pnpm --filter studio-ui dev",
    "beforeBuildCommand": "pnpm --filter studio-ui build"
  },
  "app": {
    "windows": [
      {
        "title": "Siragugal Film Studio — சிறகுகள் ஃபிலிம் ஸ்டுடியோ",
        "width": 1440,
        "height": 900,
        "minWidth": 1024,
        "minHeight": 720,
        "resizable": true,
        "fullscreen": false,
        "transparent": false,
        "decorations": true,
        "center": true
      }
    ],
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: asset:; media-src 'self' blob: asset:; font-src 'self' data:;"
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "macOS": {
      "frameworks": [],
      "minimumSystemVersion": "12.0",
      "exceptionDomain": "",
      "signingIdentity": null,
      "entitlements": null
    }
  }
}
```

### B. `packages/sira-studio-app/Cargo.toml` Modifications
```toml
[package]
name = "sira_studio_app"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true

[[bin]]
name = "siragugal-film-studio"
path = "src/main.rs"

[dependencies]
tauri = { version = "2.0.0-rc", features = [] }
sira_types = { path = "../sira-types" }
sira_config = { path = "../sira-config" }
sira_diagnostics = { path = "../sira-diagnostics" }
sfsp_engine = { path = "../sfsp-engine" }
asset_db = { path = "../asset-db" }
sira_hal = { path = "../hal" }
sira_core = { path = "../sira-core" }
sira_ai_provider = { path = "../sira-ai-provider" }
workflow_engine = { path = "../workflow-engine" }
experience_layer = { path = "../experience-layer" }
resource_manager = { path = "../resource-manager" }
cache_manager = { path = "../cache-manager" }
plugin_runtime = { path = "../plugin-runtime" }
sira_engine_story = { path = "../sira-engine-story" }
sira_engine_character = { path = "../sira-engine-character" }
sira_engine_actor = { path = "../sira-engine-actor" }
sira_engine_scene = { path = "../sira-engine-scene" }
sira_engine_director = { path = "../sira-engine-director" }
sira_engine_cinematography = { path = "../sira-engine-cinematography" }
sira_engine_audio = { path = "../sira-engine-audio" }
sira_engine_timeline = { path = "../sira-engine-timeline" }
sira_engine_render = { path = "../sira-engine-render" }
sira_engine_asset = { path = "../sira-engine-asset" }
sira_engine_workflow = { path = "../sira-engine-workflow" }
sira_engine_packaging = { path = "../sira-engine-packaging" }
sira_engine_plugin = { path = "../sira-engine-plugin" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
uuid = { version = "1.6", features = ["v4", "v7", "serde"] }
```

### C. `packages/sira-studio-app/src/main.rs`
```rust
/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sira_studio_app::{AppLaunchConfig, StudioApplication, TauriIpcBridge};

#[tauri::command]
fn invoke_sira_ipc(command: String, payload: String) -> Result<String, String> {
    match TauriIpcBridge::dispatch_command(&command, &payload) {
        sira_types::SiraResult::Success(res) => Ok(res),
        sira_types::SiraResult::Failure(err) => Err(err.to_string()),
    }
}

fn main() {
    let launch_config = AppLaunchConfig {
        project_file_path: None,
        enable_gpu_acceleration: true,
        developer_mode: cfg!(debug_assertions),
    };

    if let Err(e) = StudioApplication::bootstrap(launch_config) {
        eprintln!("Failed to bootstrap Siragugal Film Studio: {:?}", e);
        std::process::exit(1);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![invoke_sira_ipc])
        .run(tauri::generate_context!())
        .expect("Error while running Siragugal Film Studio Tauri application");
}
```

---

## 5. Security & Product Integrity Model

1. **Strict Content Security Policy (CSP)**:
   - Disabled arbitrary external script execution (`connect-src 'self'`).
   - Allowed media protocol loaders for local asset caching (`asset:`, `blob:`).
2. **Local File Boundary**:
   - Access restricted to user-approved local project workspaces (`$APP_DATA/projects/`, `$USER_HOME/SiragugalProjects/`).
3. **Zero Mandatory Network Egress**:
   - Application binary compiles and launches completely offline. Zero telemetry, zero analytics tracking, zero authentication pings.

---

## 6. Target Platform Strategy

- **Primary Target**: macOS (`aarch64-apple-darwin` for M-series Apple Silicon, with `x86_64-apple-darwin` universal binary support).
  - Configured via `rust-toolchain.toml` target triples.
  - Native macOS menu bar and window decorations supported via Tauri 2.0 window API.
- **Development/Build Verification Host**: Windows 11 (`x86_64-pc-windows-msvc`).
  - Used for fast compilation audits and workspace validation without introducing Windows-only code paths.

---

## 7. Exact Files To Be Created / Modified

| Action | Target Path | Rationale |
| :--- | :--- | :--- |
| **[NEW]** | [`packages/sira-studio-app/tauri.conf.json`](file:///D:/SiragugalFilmStudio/packages/sira-studio-app/tauri.conf.json) | Production Tauri 2.0 runner configuration file. |
| **[NEW]** | [`packages/sira-studio-app/src/main.rs`](file:///D:/SiragugalFilmStudio/packages/sira-studio-app/src/main.rs) | Binary entry point linking `sira_studio_app` library and Tauri 2 event loop. |
| **[MODIFY]** | [`packages/sira-studio-app/Cargo.toml`](file:///D:/SiragugalFilmStudio/packages/sira-studio-app/Cargo.toml) | Declare `[[bin]]` section and add `tauri = "2.0.0-rc"` dependency. |

---

## 8. Verification & Execution Plan

1. **Verification Command**:
   ```powershell
   cargo check --package sira_studio_app
   ```
2. **Production Bundle Verification**:
   ```powershell
   pnpm --filter studio-ui build
   ```
3. **Report Generation**:
   Generate updated [`docs/governance/TAURI_DESKTOP_RUNTIME_VERIFICATION.md`](file:///D:/SiragugalFilmStudio/docs/governance/TAURI_DESKTOP_RUNTIME_VERIFICATION.md) documenting empirical compilation results.

---

## 9. Declaration

> [!IMPORTANT]
> **GOVERNANCE NOTICE**:  
> No source code or configuration files have been created or modified yet during this design review pass. Implementation will commence only upon explicit governance approval.

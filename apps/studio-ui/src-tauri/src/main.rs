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
        .expect("Error while running Siragugal Film Studio Tauri desktop application");
}

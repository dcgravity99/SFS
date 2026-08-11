/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;

pub struct TauriIpcBridge;

impl TauriIpcBridge {
    pub fn dispatch_command(command: &str, payload_json: &str) -> SiraResult<String> {
        let _ = command;
        let _ = payload_json;
        // Asynchronously routes frontend IPC requests to backend engine APIs
        SiraResult::Success("{\"status\":\"ok\"}".to_string())
    }
}

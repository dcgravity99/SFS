/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub fn generate_editing_proxy(file_path: &str, target_codec: &str) -> Result<String, String> {
    if file_path.is_empty() {
        return Err("Invalid file path".to_string());
    }

    let codec = if target_codec.is_empty() {
        "ProRes Proxy"
    } else {
        target_codec
    };
    Ok(format!(
        "proxies/generated_{}.mov",
        codec.replace(' ', "_").to_lowercase()
    ))
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

fn main() {
    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .warnings_into_errors(true)
        .file("cxx/hal_device.cpp")
        .compile("sira_hal_native");

    println!("cargo:rerun-if-changed=cxx/hal_api.h");
    println!("cargo:rerun-if-changed=cxx/hal_device.cpp");
}

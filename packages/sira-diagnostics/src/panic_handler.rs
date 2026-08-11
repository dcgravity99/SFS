/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrashCategory {
    FatalPanic,
    OomCrash,
    HalDriverReset,
    PluginSegfault,
}

pub fn register_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("CRASH [FatalPanic]: {:?}", info);
        eprintln!("{}", msg);
    }));
}

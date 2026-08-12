/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod actor;
pub mod consistency;
pub mod dictionary;
pub mod lipsync;
pub mod voice;

pub use actor::*;
pub use consistency::*;
pub use dictionary::*;
pub use lipsync::*;
pub use voice::*;

pub struct ActorEngine;

impl ActorEngine {
    pub fn new() -> Self {
        Self
    }
}

/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod actor;
pub mod voice;
pub mod lipsync;
pub mod dictionary;
pub mod consistency;

pub use actor::*;
pub use voice::*;
pub use lipsync::*;
pub use dictionary::*;
pub use consistency::*;

pub struct ActorEngine;

impl ActorEngine {
    pub fn new() -> Self {
        Self
    }
}

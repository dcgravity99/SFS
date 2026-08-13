/* ============================================================================
 * Siragugal Film Studio — Module 50: Digital Twin Actor Control Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod actor;
pub mod consistency;
pub mod dictionary;
pub mod digital_twin;
pub mod lipsync;
pub mod mocap_retarget;
pub mod voice;

pub use actor::*;
pub use consistency::*;
pub use dictionary::*;
pub use digital_twin::*;
pub use lipsync::*;
pub use mocap_retarget::*;
pub use voice::*;

pub struct ActorEngine;

impl ActorEngine {
    pub fn new() -> Self {
        Self
    }
}

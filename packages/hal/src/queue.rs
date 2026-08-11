/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueType {
    Compute,
    Transfer,
    Graphics,
}

pub struct HalQueue {
    pub queue_type: QueueType,
}

impl HalQueue {
    pub fn new(queue_type: QueueType) -> Self {
        Self { queue_type }
    }
}

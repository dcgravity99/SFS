/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct VramPool {
    total_capacity_mb: usize,
    allocated_mb: Arc<AtomicUsize>,
}

impl VramPool {
    pub fn new(capacity_mb: usize) -> Self {
        Self {
            total_capacity_mb: capacity_mb,
            allocated_mb: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn reserve(&self, requested_mb: usize) -> SiraResult<bool> {
        let current = self.allocated_mb.load(Ordering::SeqCst);
        if current + requested_mb <= self.total_capacity_mb {
            self.allocated_mb.fetch_add(requested_mb, Ordering::SeqCst);
            SiraResult::Success(true)
        } else {
            SiraResult::Success(false)
        }
    }

    pub fn release(&self, released_mb: usize) {
        let current = self.allocated_mb.load(Ordering::SeqCst);
        if current >= released_mb {
            self.allocated_mb.fetch_sub(released_mb, Ordering::SeqCst);
        }
    }
}

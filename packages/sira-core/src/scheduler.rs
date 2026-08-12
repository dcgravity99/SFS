/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::job::SiraJob;
use std::collections::VecDeque;
use std::sync::RwLock;

pub struct MultiTierScheduler {
    interactive_queue: RwLock<VecDeque<SiraJob>>,
    background_queue: RwLock<VecDeque<SiraJob>>,
    batch_queue: RwLock<VecDeque<SiraJob>>,
}

impl MultiTierScheduler {
    pub fn new() -> Self {
        Self {
            interactive_queue: RwLock::new(VecDeque::new()),
            background_queue: RwLock::new(VecDeque::new()),
            batch_queue: RwLock::new(VecDeque::new()),
        }
    }

    pub fn submit_job(&self, job: SiraJob) {
        match job.priority_policy {
            crate::job::PriorityPolicy::Interactive | crate::job::PriorityPolicy::RealTime => {
                if let Ok(mut q) = self.interactive_queue.write() {
                    q.push_back(job);
                }
            }
            crate::job::PriorityPolicy::Background | crate::job::PriorityPolicy::LowPower => {
                if let Ok(mut q) = self.background_queue.write() {
                    q.push_back(job);
                }
            }
            crate::job::PriorityPolicy::Batch => {
                if let Ok(mut q) = self.batch_queue.write() {
                    q.push_back(job);
                }
            }
        }
    }
}

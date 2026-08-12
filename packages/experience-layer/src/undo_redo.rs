/*
============================================================================
Siragugal Film Studio
Copyright (C) 2026 Siragugal Film Studio Contributors
Licensed under Apache-2.0 or MIT.
============================================================================
*/

use sira_types::SiraResult;
use std::sync::RwLock;

pub trait ExperienceCommand: Send + Sync {
    fn execute(&mut self) -> SiraResult<()>;
    fn undo(&mut self) -> SiraResult<()>;
    fn redo(&mut self) -> SiraResult<()>;
}

pub struct UniversalUndoRedo {
    max_history_limit: usize,
    undo_stack_size: RwLock<usize>,
    redo_stack_size: RwLock<usize>,
}

impl UniversalUndoRedo {
    pub fn new(max_history: usize) -> Self {
        Self {
            max_history_limit: max_history,
            undo_stack_size: RwLock::new(0),
            redo_stack_size: RwLock::new(0),
        }
    }

    pub fn execute_command(&self, mut command: Box<dyn ExperienceCommand>) -> SiraResult<()> {
        match command.execute() {
            SiraResult::Error(err) => {
                return SiraResult::Error(err);
            }

            SiraResult::Cancelled { reason } => {
                return SiraResult::Cancelled { reason };
            }

            _ => {}
        }

        if let Ok(mut count) = self.undo_stack_size.write() {
            if *count < self.max_history_limit {
                *count += 1;
            }
        }

        SiraResult::Success(())
    }

    pub fn can_undo(&self) -> bool {
        self.undo_stack_size.read().map(|c| *c > 0).unwrap_or(false)
    }

    pub fn can_redo(&self) -> bool {
        self.redo_stack_size.read().map(|c| *c > 0).unwrap_or(false)
    }
}

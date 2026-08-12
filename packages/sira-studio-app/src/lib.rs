/*
============================================================================

Siragugal Film Studio
Copyright (C) 2026 Siragugal Film Studio Contributors
Licensed under Apache-2.0 or MIT.

============================================================================
*/

pub mod bootstrap;
pub mod ipc_bridge;
pub mod menu;
pub mod shell;

pub use bootstrap::*;
pub use ipc_bridge::*;
pub use menu::*;
pub use shell::*;

use sira_types::SiraResult;

pub struct StudioApplication;

impl StudioApplication {
    pub fn bootstrap(config: AppLaunchConfig) -> SiraResult<StudioApplication> {
        match ApplicationBootstrapper::bootstrap(config) {
            SiraResult::Success(_) => SiraResult::Success(Self),

            SiraResult::PartialSuccess { warnings, .. } => SiraResult::PartialSuccess {
                data: Self,
                warnings,
            },

            SiraResult::Error(error) => SiraResult::Error(error),

            SiraResult::Progress { progress, stage } => SiraResult::Progress { progress, stage },

            SiraResult::Cancelled { reason } => SiraResult::Cancelled { reason },
        }
    }

    pub fn open_window(&self, spec: WindowSpec) -> SiraResult<()> {
        DesktopShellManager::open_window(spec)
    }

    pub fn shutdown(&self) -> SiraResult<()> {
        SiraResult::Success(())
    }
}

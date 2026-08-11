/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod bootstrap;
pub mod shell;
pub mod ipc_bridge;
pub mod menu;

pub use bootstrap::*;
pub use shell::*;
pub use ipc_bridge::*;
pub use menu::*;

use sira_types::SiraResult;

pub struct StudioApplication;

impl StudioApplication {
    pub fn bootstrap(config: AppLaunchConfig) -> SiraResult<Self> {
        ApplicationBootstrapper::bootstrap(config)?;
        SiraResult::Success(Self)
    }

    pub fn open_window(&self, spec: WindowSpec) -> SiraResult<()> {
        DesktopShellManager::open_window(spec)
    }

    pub fn shutdown(&self) -> SiraResult<()> {
        SiraResult::Success(())
    }
}

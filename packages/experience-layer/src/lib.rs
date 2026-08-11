/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod event_bus;
pub mod progress;
pub mod notifications;
pub mod background_jobs;
pub mod history;
pub mod undo_redo;
pub mod diagnostics_obs;
pub mod status_bar;
pub mod accessibility;
pub mod telemetry;

pub use event_bus::*;
pub use progress::*;
pub use notifications::*;
pub use background_jobs::*;
pub use history::*;
pub use undo_redo::*;
pub use diagnostics_obs::*;
pub use status_bar::*;
pub use accessibility::*;
pub use telemetry::*;

pub struct ExperienceLayer {
    pub event_bus: ExperienceEventBus,
    pub progress_mgr: ProgressManager,
    pub notifications_hub: NotificationCenter,
    pub background_jobs: BackgroundJobManager,
    pub activity_history: ActivityHistory,
    pub undo_redo: UniversalUndoRedo,
    pub diagnostics_obs: DiagnosticsObserver,
    pub status_bar: StatusBarService,
    pub accessibility: AccessibilityFoundation,
    pub telemetry: TelemetryIntegration,
}

impl ExperienceLayer {
    pub fn new() -> Self {
        Self {
            event_bus: ExperienceEventBus::new(1000),
            progress_mgr: ProgressManager::new(),
            notifications_hub: NotificationCenter::new(),
            background_jobs: BackgroundJobManager::new(),
            activity_history: ActivityHistory::new(),
            undo_redo: UniversalUndoRedo::new(100),
            diagnostics_obs: DiagnosticsObserver::new(),
            status_bar: StatusBarService::new(),
            accessibility: AccessibilityFoundation::new(),
            telemetry: TelemetryIntegration::new(),
        }
    }
}

/* ============================================================================
 * Siragugal Film Studio — Module 36: Project Backup & Auto-Save Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod auto_save;
pub mod backup_manager;
pub mod encryption_service;
pub mod integrity_validator;
pub mod recovery_tester;
pub mod restore_engine;

pub use auto_save::*;
pub use backup_manager::create_backup_snapshot;
pub use encryption_service::encrypt_backup_archive;
pub use integrity_validator::verify_backup_integrity;
pub use recovery_tester::run_disaster_recovery_test;
pub use restore_engine::restore_project_checkpoint;

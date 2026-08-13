/* ============================================================================
 * Siragugal Film Studio — Module 33: Multi-User Real-time Collaborative Editing Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod bandwidth_optimizer;
pub mod collab;
pub mod conflict_resolver;
pub mod region_replicator;
pub mod sync_manager;
pub mod transport_security;

pub use bandwidth_optimizer::optimize_bandwidth_allocation;
pub use collab::*;
pub use conflict_resolver::resolve_metadata_conflict;
pub use region_replicator::replicate_to_region;
pub use sync_manager::initiate_region_sync;
pub use transport_security::verify_tls_transport;

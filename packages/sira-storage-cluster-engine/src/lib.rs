/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod cluster_manager;
pub mod raft_consensus;
pub mod distributed_store;
pub mod failover_controller;
pub mod node_health;

pub use cluster_manager::{join_storage_cluster, get_cluster_status};
pub use raft_consensus::execute_leader_election;
pub use distributed_store::allocate_distributed_block;
pub use failover_controller::trigger_node_failover;
pub use node_health::node_health_status;

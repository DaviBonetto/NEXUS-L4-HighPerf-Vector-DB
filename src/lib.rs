//! NEXUS-L4: High-Performance Vector Database
//!
//! Layered Architecture:
//! - Layer 1: Interface (API/CLI)
//! - Layer 2: Engine (Orchestration)
//! - Layer 3: Core (Math Kernels)
//! - Layer 4: Storage (Persistence)

pub mod core;
pub mod engine;
pub mod interface;
pub mod storage;

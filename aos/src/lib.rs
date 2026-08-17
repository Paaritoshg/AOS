//! Agent Operating System (AOS)
//! 
//! A runtime for AI agents with deterministic enforcement boundaries.
//! Separates reasoning, policy, and execution planes.

pub mod identity;
pub mod agent;
pub mod lifecycle;
pub mod scheduler;
pub mod memory;
pub mod context;
pub mod tools;
pub mod capabilities;
pub mod policy;
pub mod enforcement;
pub mod audit;
pub mod observability;
pub mod trust;
pub mod delegation;
pub mod coordination;
pub mod cluster;
pub mod kernel;
pub mod error;

pub use identity::*;
pub use agent::*;
pub use lifecycle::*;
pub use scheduler::*;
pub use memory::*;
pub use context::*;
pub use tools::*;
pub use capabilities::*;
pub use policy::*;
pub use enforcement::*;
pub use audit::*;
pub use observability::*;
pub use trust::*;
pub use delegation::*;
pub use coordination::*;
pub use cluster::*;
pub use kernel::*;
pub use error::*;

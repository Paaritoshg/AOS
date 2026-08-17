//! Module 18: Error - Unified error handling

use thiserror::Error;

/// Identity errors
#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Credentials invalid")]
    InvalidCredentials,
    #[error("Identity not found")]
    NotFound,
}

/// Unified AOS error type
#[derive(Debug, Error)]
pub enum AosError {
    #[error("Identity error: {0}")]
    Identity(#[from] IdentityError),
    
    #[error("Lifecycle error: {0}")]
    Lifecycle(#[from] crate::lifecycle::LifecycleError),
    
    #[error("Scheduler error: {0}")]
    Scheduler(#[from] crate::scheduler::SchedulerError),
    
    #[error("Memory error: {0}")]
    Memory(#[from] crate::memory::MemoryError),
    
    #[error("Context error: {0}")]
    Context(#[from] crate::context::ContextError),
    
    #[error("Tool error: {0}")]
    Tool(#[from] crate::tools::ToolError),
    
    #[error("Capability error: {0}")]
    Capability(#[from] crate::capabilities::CapabilityError),
    
    #[error("Policy error: {0}")]
    Policy(#[from] crate::policy::PolicyError),
    
    #[error("Enforcement error: {0}")]
    Enforcement(#[from] crate::enforcement::EnforcementError),
    
    #[error("Audit error: {0}")]
    Audit(#[from] crate::audit::AuditError),
    
    #[error("Observability error: {0}")]
    Observability(#[from] crate::observability::ObservabilityError),
    
    #[error("Trust error: {0}")]
    Trust(#[from] crate::trust::TrustError),
    
    #[error("Delegation error: {0}")]
    Delegation(#[from] crate::delegation::DelegationError),
    
    #[error("Coordination error: {0}")]
    Coordination(#[from] crate::coordination::CoordinationError),
    
    #[error("Cluster error: {0}")]
    Cluster(#[from] crate::cluster::ClusterError),
    
    #[error("Kernel error: {0}")]
    Kernel(#[from] crate::kernel::KernelError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for AOS operations
pub type AosResult<T> = Result<T, AosError>;

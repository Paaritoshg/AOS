//! Module 1: Identity - Agent identification and authentication

use uuid::Uuid;

/// Unique identifier for an agent
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentId {
    pub id: Uuid,
}

/// Credentials for agent authentication
#[derive(Debug, Clone)]
pub struct AgentCredentials {
    pub agent_id: AgentId,
    // Empty implementation
}

/// Identity errors
#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Credentials invalid")]
    InvalidCredentials,
    #[error("Identity not found")]
    NotFound,
}

/// Identity provider trait
pub trait IdentityProvider {
    type Error;
    
    fn authenticate(&self, credentials: &AgentCredentials) -> Result<AgentId, Self::Error>;
    fn issue_credentials(&self, agent_id: AgentId) -> Result<AgentCredentials, Self::Error>;
    fn revoke_credentials(&self, agent_id: AgentId) -> Result<(), Self::Error>;
}

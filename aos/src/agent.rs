//! Module 2: Agent - Core agent representation and state

use crate::identity::AgentId;

/// Agent state machine states
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Initializing,
    Idle,
    Reasoning,
    Executing,
    Waiting,
    Suspended,
    Terminated,
}

/// Core agent structure
#[derive(Debug)]
pub struct Agent {
    pub id: AgentId,
    pub state: AgentState,
    pub goal: Option<String>,
    // Empty implementation
}

/// Agent configuration
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub max_memory_bytes: usize,
    pub allowed_tools: Vec<String>,
    // Empty implementation
}

/// Trait for agent behavior
pub trait AgentBehavior {
    type Error;
    
    fn reason(&mut self) -> Result<(), Self::Error>;
    fn execute(&mut self) -> Result<(), Self::Error>;
    fn suspend(&mut self) -> Result<(), Self::Error>;
    fn resume(&mut self) -> Result<(), Self::Error>;
}

use thiserror::Error;
use crate::api::steps::Step;
use crate::r#impl::execution_state;

pub struct ExecutionState {
    // todo - make this private to enforce valid transitions
    pub step_states: Vec<Step>,
}

impl ExecutionState {
    pub fn new() -> ExecutionState {
        ExecutionState { step_states: vec![] }
    }
    pub fn status(&self) -> ExecutionStatus {
        execution_state::get_execution_status(self)
    }
}

#[derive(Error, Debug)]
pub enum ExecutionStateError {
    #[error("A step with a duplicate id was appended")]
    DuplicateStepIdError,

    #[error("Invalid step transition")]
    InvalidStepTransition,

    #[error("Invalid step transition on closed step")]
    InvalidStepTransitionOnClosedStep,
}

#[derive(Debug, PartialEq)]
pub enum ExecutionStatus {
    New, // No steps established or any other state
    Error,
    Failed,
    Running,
    Finished,
}
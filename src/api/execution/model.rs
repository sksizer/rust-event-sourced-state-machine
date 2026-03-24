use thiserror::Error;
use crate::api::steps::Step;
use crate::runner::get_execution_status;

pub trait ExecutionState {
    fn new() -> Self;
    fn status(&self) -> ExecutionStatus;
    fn is_stopped(&self) -> bool;

    fn get_step_state(&self, id: &str) -> Option<&Step>;
}

pub struct DefaultExecutionState {
    // todo - make this private to enforce valid transitions
    pub step_states: Vec<Step>,
}

impl ExecutionState for DefaultExecutionState {
    fn new() -> DefaultExecutionState {
        DefaultExecutionState { step_states: vec![] }
    }
    fn status(&self) -> ExecutionStatus {
        get_execution_status(self)
    }

    fn is_stopped(&self) -> bool {
        matches!(self.status(), ExecutionStatus::Error | ExecutionStatus::Failed | ExecutionStatus::Finished)
    }

    fn get_step_state(&self, id: &str) -> Option<&Step> {
        self.step_states.iter().find(|s| s.id() == id)
    }
}

#[derive(Error, Debug)]
pub enum ExecutionStateError {
    #[error("Attempt to step transition on closed execution state")]
    TransitionOnClosedExecutionState,

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
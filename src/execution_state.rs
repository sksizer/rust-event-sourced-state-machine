
use log::error;
use thiserror::Error;

use crate::api::steps::Step;

pub struct ExecutionState {
    // todo - make this private to enforce valid transitions
    pub step_states: Vec<Step>,
}

impl ExecutionState {
    pub fn status(&self) -> ExecutionStatus {
        get_execution_status(self)
    }
}

/////// IMPLEMENTATIONS

#[derive(Error, Debug)]
pub enum ExecutionStateError {
    #[error("A step with a duplicate id was appended")]
    DuplicateStepIdError,

    #[error("Invalid step transition")]
    InvalidStepTransition,

    #[error("Invalid step transition on closed step")]
    InvalidStepTransitionOnClosedStep,
}

// TODO: Duplicate check only compares against the *last* step, not all steps.
//  Adding steps 1, 2, 1 would pass. If global uniqueness is intended,
//  check all step_states (or use a HashSet of ids alongside the Vec).
pub fn append_step_state(
    mut execution_state: ExecutionState,
    step: Step,
) -> Result<ExecutionState, ExecutionStateError> {
    if !execution_state.step_states.is_empty() {
        let prior_id = execution_state.step_states[execution_state.step_states.len() - 1]
            .id()
            .to_string();
        if prior_id == step.id() {
            return Err(ExecutionStateError::DuplicateStepIdError);
        }
    }
    execution_state.step_states.push(step);
    Ok(execution_state)
}

// TODO: This replaces the last step unconditionally without verifying the id matches.
//  An event for step "2" would silently overwrite step "1" if "1" is last.
//  Consider verifying step_state.id matches the last step's id.
pub fn update_step_state(
    mut execution_state: ExecutionState,
    step: Step,
) -> Result<ExecutionState, ExecutionStateError> {
    if execution_state.step_states.is_empty() {
        error!("Attempt to transition a step on an empty execution_state step list");
        return Err(ExecutionStateError::InvalidStepTransition);
    }

    if execution_state.step_states[execution_state.step_states.len() - 1].is_closed() {
        return Err(ExecutionStateError::InvalidStepTransitionOnClosedStep);
    }

    execution_state.step_states.pop();
    execution_state.step_states.push(step);
    Ok(execution_state)
}

#[derive(Debug, PartialEq)]
pub enum ExecutionStatus {
    New, // No steps established or any other state
    Error,
    Failed,
    Running,
    Finished,
}

fn get_execution_status(execution_state: &ExecutionState) -> ExecutionStatus {
    if execution_state.step_states.is_empty() {
        return ExecutionStatus::New;
    }
    if execution_state.step_states.iter().all(|s| s.is_completed()) {
        return ExecutionStatus::Finished;
    }
    if execution_state.step_states.iter().any(|s| s.is_failed()) {
        return ExecutionStatus::Failed;
    }
    if execution_state.step_states.iter().any(|s| s.is_error()) {
        return ExecutionStatus::Error;
    }
    ExecutionStatus::Running
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::steps::{StepCore, SyncStep, AsyncStep};

    fn sync_core(id: &str) -> StepCore {
        StepCore { id: id.to_string(), kind: "alpha".to_string(), input: None }
    }

    #[test]
    fn test_execution_status() {
        let execution_state = ExecutionState {
            step_states: vec![],
        };
        assert_eq!(execution_state.status(), ExecutionStatus::New);
    }

    #[test]
    fn append_step_with_duplicate_id_error() {
        let execution_state = ExecutionState {
            step_states: vec![Step::Async(AsyncStep::Running(sync_core("1")))],
        };
        let result = append_step_state(
            execution_state,
            Step::Sync(SyncStep::Ready(sync_core("1"))),
        );
        assert!(result.is_err());
    }

    #[test]
    fn updating_finished_step_error() {
        let execution_state = ExecutionState {
            step_states: vec![Step::Sync(SyncStep::Completed { core: sync_core("1"), output: None })],
        };

        let result = update_step_state(
            execution_state,
            Step::Sync(SyncStep::Ready(sync_core("1"))),
        );
        assert!(result.is_err());
    }
}

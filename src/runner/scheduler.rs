use crate::execution_state::ExecutionState;
use crate::api::steps::StepState;

/// Given an execution state (and later a system state) this determines what step to take and
/// produces an event that can be persisted
pub fn scheduler(execution_state: &ExecutionState) -> Option<&StepState> {
    execution_state.step_states.iter().find(|s| s.is_runnable())
}

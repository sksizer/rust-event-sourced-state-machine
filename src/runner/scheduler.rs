use crate::api::execution::ExecutionState;
use crate::api::steps::Step;

/// Given an execution state (and later a system state) this determines what step to take and
/// produces an event that can be persisted
pub fn scheduler(execution_state: &ExecutionState) -> Option<&Step> {
    execution_state.step_states.iter().find(|s| s.is_runnable())
}

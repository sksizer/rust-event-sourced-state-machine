use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::steps::{AsyncStep, Step, SyncStep};
use crate::runner::reduce::get_step_core;

pub(super) fn complete_step(execution_state: DefaultExecutionState, id: &str) -> DefaultExecutionState {
    let step = execution_state.get_step_state(id);
    execution_state
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complete_step() {
    }
}
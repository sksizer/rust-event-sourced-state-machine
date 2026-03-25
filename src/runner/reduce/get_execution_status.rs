use crate::api::execution::{DefaultExecutionState, ExecutionStatus};

pub fn get_execution_status(execution_state: &DefaultExecutionState) -> ExecutionStatus {
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
mod test {
    use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStatus};

    #[test]
    fn test_execution_status() {
        let execution_state = DefaultExecutionState {
            step_states: vec![],
        };
        assert_eq!(execution_state.status(), ExecutionStatus::New);
    }
}

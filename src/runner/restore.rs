//! Creates a realized execution state from an event stream
use crate::execution_state::ExecutionState;
use crate::steps::types::{EventStream, StepEvent};
use crate::runner::reduce::reduce;

/// helper function to return a single execution state over a series of events
pub fn restore(event_stream: EventStream) -> ExecutionState {
    let execution_state = ExecutionState {
        step_states: Vec::new(),
    };
    event_stream.iter().fold(execution_state, reduce)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::execution_state::ExecutionStatus;
    use crate::steps::types::{StepState, StepKind};

    #[test]
    fn test_adding_a_single_step() {
        let event_stream = vec![
            StepEvent::Add(String::from("1"), StepKind::Sync("alpha".to_string())),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert_eq!(execution_state.step_states[0].id(), "1");
    }

    #[test]
    fn test_adding_multiple_steps() {
        let event_stream = vec![
            StepEvent::Add(String::from("1"), StepKind::Sync("alpha".to_string())),
            StepEvent::Add(String::from("2"), StepKind::Sync("beta".to_string())),
            StepEvent::Add(String::from("3"), StepKind::Sync("gamma".to_string())),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 3);
    }

    #[test]
    fn single_step_progression() {
        let event_stream = vec![
            StepEvent::Add(String::from("1"), StepKind::Sync("alpha".to_string())),
            StepEvent::Start(String::from("1")),
            StepEvent::Complete(String::from("1"), None),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert!(matches!(execution_state.step_states[0], StepState::Completed { .. }));
        assert_eq!(execution_state.step_states[0].id(), "1");
    }

    #[test]
    fn two_step_progression() {
        let event_stream = vec![
            StepEvent::Add(String::from("1"), StepKind::Sync("alpha".to_string())),
            StepEvent::Start(String::from("1")),
            StepEvent::Complete(String::from("1"), None),
            StepEvent::Add(String::from("2"), StepKind::Sync("beta".to_string())),
            StepEvent::Start(String::from("2")),
            StepEvent::Complete(String::from("2"), None),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 2);
        assert!(matches!(execution_state.step_states[0], StepState::Completed { .. }));
        assert!(matches!(execution_state.step_states[1], StepState::Completed { .. }));
    }

    #[test]
    fn three_step_failure() {
        let event_stream = vec![
            StepEvent::Add(String::from("1"), StepKind::Sync("alpha".to_string())),
            StepEvent::Start(String::from("1")),
            StepEvent::Complete(String::from("1"), None),
            StepEvent::Add(String::from("2"), StepKind::Sync("beta".to_string())),
            StepEvent::Start(String::from("2")),
            StepEvent::Complete(String::from("2"), None),
            StepEvent::Add(String::from("3"), StepKind::Sync("gamma".to_string())),
            StepEvent::Start(String::from("3")),
            StepEvent::Failed(String::from("3"), Some("something went wrong".into())),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 3);
        assert!(matches!(execution_state.step_states[0], StepState::Completed { .. }));
        assert!(matches!(execution_state.step_states[1], StepState::Completed { .. }));
        assert!(matches!(execution_state.step_states[2], StepState::Failed { .. }));

        assert_eq!(execution_state.status(), ExecutionStatus::Failed);
    }
}

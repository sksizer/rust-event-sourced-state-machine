use crate::execution_state;
use crate::execution_state::ExecutionState;
use crate::steps::types::{StepCore, StepEvent, StepState};

/// Takes prior state + an event and returns an updated state
pub fn reduce(execution_state: ExecutionState, step_event: &StepEvent) -> ExecutionState {
    match step_event {
        StepEvent::Add(id, kind) => {
            let r = execution_state::append_step_state(
                execution_state,
                StepState::Ready(StepCore {
                    id: id.clone(),
                    kind: kind.clone(),
                    input: None,
                }),
            );
            // TODO: propagate error instead of unwrap
            r.unwrap()
        }
        StepEvent::Start(id) => {
            let core = get_step_core(&execution_state, id);
            execution_state::update_step_state(
                execution_state,
                StepState::Running(core),
            )
            .unwrap()
        }
        StepEvent::Complete(id, output) => {
            let core = get_step_core(&execution_state, id);
            execution_state::update_step_state(
                execution_state,
                StepState::Completed { core, output: output.clone() },
            )
            .unwrap()
        }
        StepEvent::Failed(id, failure) => {
            let core = get_step_core(&execution_state, id);
            execution_state::update_step_state(
                execution_state,
                StepState::Failed { core, failure: failure.clone() },
            )
            .unwrap()
        }
        StepEvent::Error(id, failure) => {
            let core = get_step_core(&execution_state, id);
            execution_state::update_step_state(
                execution_state,
                StepState::Error { core, failure: failure.clone() },
            )
            .unwrap()
        }
    }
}

/// Extract the core from the current step being transitioned
fn get_step_core(execution_state: &ExecutionState, id: &str) -> StepCore {
    let last = execution_state.step_states.last()
        .expect("Cannot transition step on empty execution state");
    assert_eq!(last.id(), id, "Event id does not match current step id");
    last.core().clone()
}

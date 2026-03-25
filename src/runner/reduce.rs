mod add_step;
mod get_execution_status;
mod update_step;

use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::steps::Step;
use crate::api::steps::{AsyncStep, StepCore, StepEvent, SyncStep};
use serde_json::Value;

use add_step::append_step_state;
pub use get_execution_status::get_execution_status;
use update_step::update;

/// Takes prior state + an event and returns an updated state
pub fn reduce(
    execution_state: DefaultExecutionState,
    step_event: &StepEvent,
) -> DefaultExecutionState {
    match step_event {
        StepEvent::AddSync(payload) => {
            // TODO - this probably doesn't make sense to just transition a step from New to Ready in the reducer for no other reason than being in the new state
            // READY should probably mean that all the data is present for a step - which in many cases is going to mean that it's dependencies and prior steps are also ready and have data prepped
            append_step_state(
                execution_state,
                Step::Sync(SyncStep::Ready {
                    core: StepCore {
                        id: payload.id.clone(),
                        kind: payload.kind.clone(),
                        config: payload.config.clone(),
                    },
                    input: None,
                }),
            )
            // TODO: propagate error instead of unwrap
            .unwrap()
        }
        StepEvent::AddAsync(payload) => append_step_state(
            execution_state,
            Step::Async(AsyncStep::Ready(StepCore {
                id: payload.id.clone(),
                kind: payload.kind.clone(),
                config: payload.config.clone(),
            })),
        )
        .unwrap(),
        StepEvent::Start(id, input) => {
            let core = get_step_core(&execution_state, id);
            let is_async = matches!(execution_state.get_step_state(id).unwrap(), Step::Async(_));
            if is_async {
                update(
                    execution_state,
                    Step::Async(AsyncStep::Running {
                        core,
                        input: input.clone(),
                    }),
                )
            } else {
                update(
                    execution_state,
                    Step::Sync(SyncStep::Ready {
                        core,
                        input: input.clone(),
                    }),
                )
            }
            .unwrap()
        }
        StepEvent::Complete(payload) => {
            let core = get_step_core(&execution_state, &payload.id);
            let input = get_step_input(&execution_state, &payload.id);
            match execution_state.get_step_state(&payload.id) {
                Some(step) => if matches!(step, Step::Async(_)) {
                    update(
                        execution_state,
                        Step::Async(AsyncStep::Completed {
                            core,
                            input,
                            output: payload.output.clone(),
                        }),
                    )
                } else {
                    update(
                        execution_state,
                        Step::Sync(SyncStep::Completed {
                            core,
                            input,
                            output: payload.output.clone(),
                        }),
                    )
                }
                .unwrap(),
                None => panic!("Step not found in execution state"),
            }
        }
        StepEvent::Failed(payload) => {
            let core = get_step_core(&execution_state, &payload.id);
            let input = get_step_input(&execution_state, &payload.id);
            match execution_state.get_step_state(&payload.id) {
                Some(step) => if matches!(step, Step::Async(_)) {
                    update(
                        execution_state,
                        Step::Async(AsyncStep::Failed {
                            core,
                            input,
                            failure: payload.reason.clone(),
                        }),
                    )
                } else {
                    update(
                        execution_state,
                        Step::Sync(SyncStep::Failed {
                            core,
                            input,
                            failure: payload.reason.clone(),
                        }),
                    )
                }
                .unwrap(),
                None => panic!("Step not found in execution state"),
            }
        }
        StepEvent::Error(payload) => {
            let core = get_step_core(&execution_state, &payload.id);
            let input = get_step_input(&execution_state, &payload.id);
            match execution_state.get_step_state(&payload.id) {
                Some(step) => if matches!(step, Step::Async(_)) {
                    update(
                        execution_state,
                        Step::Async(AsyncStep::Error {
                            core,
                            input,
                            failure: payload.reason.clone(),
                        }),
                    )
                } else {
                    update(
                        execution_state,
                        Step::Sync(SyncStep::Error {
                            core,
                            input,
                            failure: payload.reason.clone(),
                        }),
                    )
                }
                .unwrap(),
                None => panic!("Step not found in execution state"),
            }
        }
    }
}

fn get_step_core(execution_state: &DefaultExecutionState, id: &str) -> StepCore {
    execution_state
        .step_states
        .iter()
        .find(|s| s.id() == id)
        .unwrap_or_else(|| panic!("Step {} not found in execution state", id))
        .core()
        .clone()
}

fn get_step_input(execution_state: &DefaultExecutionState, id: &str) -> Option<Value> {
    execution_state
        .step_states
        .iter()
        .find(|s| s.id() == id)
        .and_then(|s| s.input())
        .cloned()
}

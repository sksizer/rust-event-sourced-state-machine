//! Stateful object that ties together processing loop with functions

use std::thread::sleep;
use log::trace;
use crate::api::events::EventStream;
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::runner::{executor, reduce, restore, scheduler};
use crate::runner::registry::Registry;
pub struct Controller {
    registry: Registry,
    event_stream: EventStream
}

impl Controller {
    pub fn new(registry: Registry, event_stream: Option<EventStream>) -> Controller {
        Controller {
            registry,
            event_stream: event_stream.unwrap_or_else(EventStream::new),
        }
    }

    pub fn start(&self) -> DefaultExecutionState {
        // Step 1. Restore Execution State
        // - reduces an up-to-date execution state from the event stream
        let mut execution_state = restore(&self.event_stream);

        // Step 2. Kick off processing from current state;
        while !execution_state.is_stopped() {
            trace!("Controller - processing");
            // sleep(std::time::Duration::from_millis(1000))
            let next_step = scheduler(&execution_state);
            let event = executor(&self.registry, next_step.unwrap());
            execution_state = reduce(execution_state, &event);
        }
        execution_state
    }
}


#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::api::steps::StepEvent;
    use super::*;
    #[test]
    fn test_controller_start() {
        let event_stream = vec![
            StepEvent::add_sync("1", "echo", Some(json!({ "message": "hello" }))),
            StepEvent::add_sync("1", "echo", None),
            StepEvent::add_sync("1", "echo", None),
            StepEvent::add_sync("1", "echo", None),
            StepEvent::add_sync("1", "echo", None),
            StepEvent::add_sync("1", "echo", None),
            StepEvent::add_sync("1", "echo", None),
            StepEvent::add_sync("1", "echo", None)
        ];
        let controller = Controller::new(Registry::new(None, None), None);
        controller.start();
    }
}
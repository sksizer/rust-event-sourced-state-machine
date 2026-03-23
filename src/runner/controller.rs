//! Stateful object that ties together processing loop with functions

use crate::api::events::EventStream;
use crate::runner::{reduce, restore};
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

    pub fn start(&self) {
        // Step 1. Restore Execution State
        // - reduces an up-to-date execution state from the event stream
        let execution_state = restore(&self.event_stream);


        // Step 2. Kick off ongoing processing (perhaps an event?)
        // - receive an event
        // - persist the state
        // - process the event - get new execution state
        // - receive or emit events from processing and new state
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
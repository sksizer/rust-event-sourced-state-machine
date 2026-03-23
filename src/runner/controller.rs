//! Stateful object that ties together processing loop with functions

use crate::api::events::EventStream;
use crate::runner::registry::Registry;
pub struct Controller {
    registry: Registry,
    event_stream: EventStream
}

impl Controller {
    pub fn new(registry: Registry, event_stream: EventStream) -> Controller {
        Controller {
            registry,
            event_stream
        }
    }

    pub fn start(&self) {
        // Step 1. Restore Execution State

    }
}
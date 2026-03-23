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
        // - reduces an up-to-date execution state from the event stream

        // Step 2. Kick off ongoing processing (perhaps an event?)
        // - receive an event
        // - persist the state
        // - process the event - get new execution state
        // - recieve or emit events from processing and new state
    }
}
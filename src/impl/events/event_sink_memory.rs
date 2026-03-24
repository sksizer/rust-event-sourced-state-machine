use crate::api::events::EventSink;
use crate::api::steps::StepEvent;

pub struct InMemoryEventSink {
    pub events: Vec<StepEvent>,
}
impl EventSink for InMemoryEventSink {
    fn new() -> Self {
        InMemoryEventSink { events: vec![] }
    }
    fn send_event(&mut self, event: StepEvent) -> bool {
        self.events.push(event);
        true
    }
}

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
        // let mut execution_state = runner::restore(self.event_stream);
        // println!("Execution Status: {:?}", execution_state.status());
        // view::summarize::execution_state(&execution_state);
        //
        // execution_state = runner::reduce(execution_state, &StepEvent::Complete("1".to_string(), None));
        // println!("Execution Status: {:?}", execution_state.status());
        // view::summarize::execution_state(&execution_state);
        //
        // execution_state = runner::reduce(execution_state, &StepEvent::Add("2".to_string(), StepKind::Shell));
        // println!("Execution Status: {:?}", execution_state.status());
        // view::summarize::execution_state(&execution_state);
        //
        // let next_step = runner::scheduler(&execution_state).unwrap();
        // let result_event = runner::executor(next_step);
        // execution_state = runner::reduce(execution_state, &result_event);
        // println!("Execution Status: {:?}", execution_state.status());
        // view::summarize::execution_state(&execution_state);
    }
}
use steps::types::EventStream;
use steps::types::{StepEvent, StepKind};

mod execution_state;
mod steps;
mod view;
mod runner;
mod fixtures;

use runner::Registry;

fn main() {
    let registry = Registry::new(Some(fixtures::test_step_modules()));

    let event_stream: EventStream = vec![
        StepEvent::Add("1".to_string(), StepKind::Sync("1".to_string())),
        StepEvent::Start("1".to_string()),
    ];
    let mut execution_state = runner::restore(event_stream);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    execution_state = runner::reduce(execution_state, &StepEvent::Complete("1".to_string(), None));
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    execution_state = runner::reduce(execution_state, &StepEvent::Add("2".to_string(), StepKind::Sync("2".to_string())));
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    let next_step = runner::scheduler(&execution_state).unwrap();
    let result_event = runner::executor(&registry, next_step);
    execution_state = runner::reduce(execution_state, &result_event);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);
}

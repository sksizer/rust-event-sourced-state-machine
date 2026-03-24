use log::trace;
use serde_json::json;
use api::events::EventStream;

mod steps;
mod view;
mod runner;
mod fixtures;
mod api;
mod r#impl;

use runner::Registry;
use crate::api::execution::ExecutionState;
use crate::api::steps::StepEvent;
use crate::fixtures::get_registry;
use crate::runner::Controller;


fn main() {
    pretty_env_logger::init();
    let registry = Registry::new(Some(fixtures::get_test_step_modules()), None);
    let event_stream: EventStream = vec![
        StepEvent::AddSync("1".to_string(), "echo".to_string(), Some(json!("echo"))),
    ];
    let mut execution_state = runner::restore(&event_stream);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    let next_step = runner::scheduler(&execution_state).unwrap();
    let result_event = runner::executor(&registry, next_step);
    execution_state = runner::reduce(execution_state, &result_event);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    execution_state = runner::reduce(execution_state, &(StepEvent::add_sync("2", "echo",  Some(json!("echo")))));
    let next_step = runner::scheduler(&execution_state).unwrap();
    let result_event = runner::executor(&registry, next_step);
    execution_state = runner::reduce(execution_state, &result_event);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    example_one();
}

fn example_one() {
    trace!("Example 1");
    let event_stream : EventStream = vec![
        StepEvent::add_sync("1", "echo", Some(json!("echo"))),
        StepEvent::add_sync("2", "echo", Some(json!("echo"))),
        StepEvent::add_sync("3", "echo", Some(json!("echo"))),
        StepEvent::add_sync("4", "echo", Some(json!("echo"))),
        StepEvent::add_sync("5", "echo", Some(json!("echo"))),
        StepEvent::add_sync("6", "echo", Some(json!("echo"))),
    ];

    let controller = Controller::new(
        get_registry(),
        Some(event_stream));
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}

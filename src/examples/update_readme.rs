use evented_worker::api::steps::StepEvent;
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::view;
use log::trace;
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

pub fn update_readme() {
    trace!("Example 3: Update Readme");
    let event_log = Rc::new(RefCell::new(vec![
        StepEvent::add_sync(
            "0",
            "shell",
            Some(
                json!({ "commands": ["pnpm dlx @anthropic-ai/claude-code -p \"Please update a good README.md for this project\""] }),
            ),
        ),
        StepEvent::add_sync("1", "echo", None),
    ]));
    let mut controller = Controller::new(get_registry(), event_log.clone());
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}

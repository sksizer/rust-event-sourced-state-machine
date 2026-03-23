use serde_json::Value;
use crate::api::steps::{SyncStepHandler, StepConfig, StepInput};

static NAME: &str = "shell";

fn validate_config(_: Option<Value>) -> Result<(), String> { Ok(()) }
fn validate_input(_: Option<Value>) -> Result<(), String> { Ok(()) }

fn shell_handler(_config: StepConfig, input: StepInput) -> Result<Value, Vec<String>> {
    let _output = std::process::Command::new("ls").output();
    println!("Shell Module - input: {:?}", input.0);
    Ok(Value::Null)
}

pub fn get_shell_module() -> SyncStepHandler {
    SyncStepHandler {
        name: "Synchronous Shell Step".to_string(),
        id: NAME.to_string(),
        description: "Executes a shell command synchronously".to_string(),
        validate_config: Some(validate_config),
        validate_input: Some(validate_input),
        handler: shell_handler,
    }
}

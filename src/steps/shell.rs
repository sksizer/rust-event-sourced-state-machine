use serde_json::Value;
use serde::Deserialize;
use crate::api::steps::{SyncStepHandler, StepConfig, StepInput, StepError};

static NAME: &str = "shell";

fn validate_config(config: Option<Value>) -> Result<(), StepError> {
    match config {
        None => Err(StepError::InvalidConfig("config is required".to_string())),
        Some(value) => get_config(value)
            .map(|_| ())
            .map_err(|e| StepError::InvalidConfig(e.to_string())),
    }
}

fn get_config(value: Value) -> Result<Config, serde_json::Error> {
    serde_json::from_value(value)
}

fn validate_input(_: Option<Value>) -> Result<(), String> { Ok(()) }

#[derive(Deserialize)]
struct Config {
    commands: Vec<String>,
}

fn shell_handler(config: StepConfig, input: StepInput) -> Result<Value, Vec<String>> {
    let config = get_config(config.0.unwrap()).unwrap();
    let mut results : Vec<String> = vec![];
    config.commands.iter().for_each(|command| {
        println!("Executing command: {}", command);
        let _output = std::process::Command::new(command).output();
        println!("Command executed {}", String::from_utf8_lossy(&_output.unwrap().stdout));
        results.push(command.to_string());
    });
    Ok(Value::Array(results.into_iter().map(|s| Value::String(s)).collect()))
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

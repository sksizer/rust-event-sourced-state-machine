use std::pin::Pin;
use serde_json::Value;
use super::errors::StepError;

pub struct StepConfig(pub Option<Value>);
pub struct StepInput(pub Option<Value>);

pub type ValidateConfig = fn(Option<Value>) -> Result<(), StepError>;
pub type ValidateInput = fn(Option<Value>) -> Result<(), String>;

type SyncHandler = fn(StepConfig, StepInput) -> Result<Value, Vec<String>>;

pub struct SyncStepHandler {
    pub name: String,
    pub id: String,
    pub description: String,
    pub validate_config: Option<ValidateConfig>,
    pub validate_input: Option<ValidateInput>,
    pub handler: SyncHandler,
}

type AsyncHandler = fn(StepConfig, StepInput) -> Pin<Box<dyn std::future::Future<Output = Result<Value, Vec<String>>> + Send>>;

pub struct AsyncStepHandler {
    pub name: String,
    pub id: String,
    pub description: String,
    pub validate_config: Option<ValidateConfig>,
    pub validate_input: Option<ValidateInput>,
    pub handler: AsyncHandler,
}

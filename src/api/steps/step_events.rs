use serde_json::Value;
use crate::api::steps::{StepId, StepKind};

// An event indicates when something HAS happened — and should result in some state change
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum StepEvent {
    AddSync(StepId, StepKind, Option<Value>),
    AddAsync(StepId, StepKind, Option<Value>),
    Start(StepId),
    Complete(StepId, Option<Value>),
    Failed(StepId, Option<String>),
    Error(StepId, Option<String>),
}

impl StepEvent {
    pub fn add_sync(id: impl ToString, kind: impl ToString, input: Option<Value>) -> Self {
        StepEvent::AddSync(id.to_string(), kind.to_string(), input)
    }
    pub fn add_async(id: impl ToString, kind: impl ToString, input: Option<Value>) -> Self {
        StepEvent::AddAsync(id.to_string(), kind.to_string(), input)
    }
}
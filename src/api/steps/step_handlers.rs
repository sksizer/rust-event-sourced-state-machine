use serde_json::Value;

pub struct SyncStepHandler {
    /// Name for the step
    pub name: String,
    pub id: String,
    pub description: String,
    pub handler: fn(Option<Value>) -> Option<Value>,
}

pub struct AsyncStepHandler {
    /// Name for the step
    pub name: String,
    pub id: String,
    pub description: String,
    pub handler: fn(Option<Value>) -> String,
}
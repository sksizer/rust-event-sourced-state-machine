use serde_json::Value;
pub type StepId = String;
pub type StepKind = String;

#[derive(Clone, Debug)]
pub struct StepCore {
    pub id: StepId,
    pub kind: StepKind,
    pub input: Option<Value>,
}
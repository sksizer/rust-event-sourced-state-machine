
type StepKind = String;

#[derive(Clone, Debug)]
pub struct StepCore {
    pub id: String,
    pub kind: StepKind,
    pub input: Option<String>,
}

#[derive(Clone, Debug)]
pub enum SyncStep {
    Ready(StepCore),
    Completed {core: StepCore, output: Option<String>},
    Failed {core: StepCore, failure: Option<String>},
    Error {core: StepCore, failure: Option<String>},
}
#[derive(Clone, Debug)]
pub enum ASyncStep {
    Ready(StepCore),
    Running(StepCore),
    Completed {core: StepCore, output: Option<String>},
    Failed {core: StepCore, failure: Option<String>},
    Error {core: StepCore, failure: Option<String>},
}

#[derive(Clone, Debug)]
pub enum Step {
    Sync(SyncStep),
    Async(ASyncStep)
}
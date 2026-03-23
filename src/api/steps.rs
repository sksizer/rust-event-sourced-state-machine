mod types;

pub type StepId = String;

#[derive(Clone, Debug)]
pub enum StepKind {
    /// Identifies which sync step it is
    Sync(String),
}

#[derive(Clone, Debug)]
pub struct StepCore {
    pub id: StepId,
    pub kind: StepKind,
    pub input: Option<String>,
}

#[derive(Debug)]
pub enum StepState {
    Ready(StepCore),
    Running(StepCore),
    Completed { core: StepCore, output: Option<String> },
    Failed { core: StepCore, failure: Option<String> },
    Error { core: StepCore, failure: Option<String> },
}

impl StepState {
    pub fn core(&self) -> &StepCore {
        match self {
            StepState::Ready(core) => core,
            StepState::Running(core) => core,
            StepState::Completed { core, .. } => core,
            StepState::Failed { core, .. } => core,
            StepState::Error { core, .. } => core,
        }
    }

    pub fn id(&self) -> &str {
        &self.core().id
    }

    pub fn kind(&self) -> &StepKind {
        &self.core().kind
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, StepState::Completed { .. } | StepState::Failed { .. } | StepState::Error { .. })
    }

    pub fn is_runnable(&self) -> bool {
        matches!(self, StepState::Ready(_) | StepState::Running(_))
    }
}



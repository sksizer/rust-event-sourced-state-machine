use log::trace;
use crate::runner::registry::Registry;
use crate::steps::types::{StepEvent, StepKind, StepState};

/// Knows how to call a step function. Is at the edge of side effects
pub fn executor(registry:&Registry , step: &StepState) -> StepEvent {
    trace!("executor - step: {:?}", step);
    let id = step.id().to_string();
    let kind = step.kind();
    match kind {
        StepKind::Sync(kind) => {
            let step_mod = registry.get_sync_module(kind);
            match step_mod {
                Some(s) => {
                    let r = (s.handler)();
                    StepEvent::Complete(id, Some(r))
                }
                None => {
                    StepEvent::Error(id, Some("No step handler registered".to_string()))
                }
            }
        }
    }
}
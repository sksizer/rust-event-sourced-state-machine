use log::trace;
use crate::api::events::StepEvent;
use crate::api::steps::{Step, SyncStep};
use crate::runner::registry::Registry;

/// Knows how to call a step function. Is at the edge of side effects
pub fn executor(registry: &Registry, step: &Step) -> StepEvent {
    trace!("executor - step: {:?}", step);
    let id = step.id().to_string();
    match step {
        Step::Sync(s) => {
            let kind = match s {
                SyncStep::Ready(core) => &core.kind,
                _ => return StepEvent::Error(id, Some("Sync step is not in Ready state".to_string())),
            };
            match registry.get_sync_module(kind) {
                Some(m) => StepEvent::Complete(id, Some((m.handler)())),
                None => StepEvent::Error(id, Some("No step handler registered".to_string())),
            }
        }
        Step::Async(_) => {
            unimplemented!("Async step execution not yet implemented")
        }
    }
}

use crate::api::steps::{AsyncStepHandler, SyncStepHandler};
pub struct Registry {
    /// Step modules that handle synchronous operations
    sync_definitions: Vec<SyncStepHandler>,
    async_definitions:Vec<AsyncStepHandler>
}

impl Registry {
    pub fn new(sync_step_modules: Option<Vec<SyncStepHandler>>, async_step_modules:Option<Vec<AsyncStepHandler>>) -> Registry {
        Registry {
            sync_definitions: sync_step_modules.unwrap_or_else(Vec::new),
            async_definitions:async_step_modules.unwrap_or_else(Vec::new)
        }
    }

    pub fn register_sync(&mut self, step: SyncStepHandler) -> Result<(), String > {
        if self.get_sync_module(&step.id).is_some() {
            return Err(format!("Step with id {} already exists", step.id));
        }
        self.sync_definitions.push(step);
        Ok(())
    }

    pub fn register_async(&mut self, step: AsyncStepHandler) -> Result<(), String> {
        if self.get_async_module(&step.id).is_some() {
            return Err(format!("Step with id {} already exists", step.id));
        }
        self.async_definitions.push(step);
        Ok(())
    }

    pub fn get_sync_module(&self, step_kind:&str) -> Option<&SyncStepHandler> {
        self.sync_definitions.iter().find(|s| s.id == step_kind)
    }

    pub fn get_async_module(&self, step_kind:&str) -> Option<&AsyncStepHandler> {
        self.async_definitions.iter().find(|s| s.id == step_kind)
    }
}


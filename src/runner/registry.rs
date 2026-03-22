pub struct Registry {
    /// Step modules that handle synchronous operations
    sync_definitions: Vec<SyncStepModule>
}

impl Registry {
    pub fn new(starting_modules: Option<Vec<SyncStepModule>>) -> Registry {
        Registry {
            sync_definitions: starting_modules.unwrap_or_else(Vec::new)
        }
    }

    pub fn register_sync(&mut self, step: SyncStepModule) {
        self.sync_definitions.push(step);
    }

    pub fn get_sync_module(&self, name:&str) -> Option<&SyncStepModule> {
        self.sync_definitions.iter().find(|s| s.name == name)
    }
}

pub struct SyncStepModule {
    /// Name for the step
    pub name: String,
    pub id: String,
    pub description: String,
    pub handler: fn() -> String,
}
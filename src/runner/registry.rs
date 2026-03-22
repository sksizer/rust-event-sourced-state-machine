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

    // TODO
    // - [ ] Allow an ID override
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

pub fn test_modules() -> Vec<SyncStepModule> {
    vec![
        SyncStepModule {
            name: "Alpha".to_string(),
            id: "alpha".to_string(),
            description: "Alpha test module".to_string(),
            handler: || "alpha".to_string(),
        },
        SyncStepModule {
            name: "Beta".to_string(),
            id: "beta".to_string(),
            description: "Beta test module".to_string(),
            handler: || "beta".to_string(),
        },
        SyncStepModule {
            name: "Gamma".to_string(),
            id: "gamma".to_string(),
            description: "Gamma test module".to_string(),
            handler: || "gamma".to_string(),
        }
    ]
}
use crate::runner::SyncStepModule;

pub fn get_shell_module() -> SyncStepModule {
    SyncStepModule {
        name: "Synchronous Shell Step".to_string(),
        id: "sync_shell_step".to_string(),
        description: "Executes a shell command synchronously".to_string(),
        handler: || {
            "Shell Module".to_string()
        }
    }
}

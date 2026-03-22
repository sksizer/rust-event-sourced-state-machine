use crate::runner::SyncStepModule;
use crate::steps::{get_echo_module, get_shell_module};

pub fn test_step_modules() -> Vec<SyncStepModule> {
    vec![
        get_shell_module(),
        get_echo_module(),
    ]
}
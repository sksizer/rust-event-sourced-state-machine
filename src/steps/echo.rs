//! This is a test step module that basically just passes the input through to output
use crate::runner::SyncStepModule;

// TODO - implement actual echoing for testing
pub fn get_echo_module() -> SyncStepModule {
    SyncStepModule {
        name: "Synchronous Echo Step".to_string(),
        id: "sync_echo_step".to_string(),
        description: "Passes input to output synchronously".to_string(),
        handler: || {
            "Echo Module".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_echo_module() {
        let modules = get_echo_module();
        assert_eq!((modules.handler)(), "Echo Module".to_string());
    }
}

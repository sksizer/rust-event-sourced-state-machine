//! Models event sources
use crate::api::steps::Event;

pub type EventStream = Vec<Event>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_sync() {
        let step_event = Event::add_sync("1", "echo", None);
        assert_eq!(step_event, Event::add_sync("1", "echo", None));
    }

    #[test]
    fn test_add_async() {
        let step_event = Event::add_async("1", "echo", None);
        assert_eq!(step_event, Event::add_async("1", "echo", None));
    }
}

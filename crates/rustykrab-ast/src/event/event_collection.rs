//! Event collection.

use crate::event::EventHandler;

/// Ordered collection of events.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EventCollection {
    events: Vec<EventHandler>,
}

impl EventCollection {
    /// Creates empty collection.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Adds an event.
    pub fn add(&mut self, event: EventHandler) {
        self.events.push(event);
    }

    /// Removes event by index.
    pub fn remove(&mut self, index: usize) -> Option<EventHandler> {
        if index >= self.events.len() {
            return None;
        }

        Some(self.events.remove(index))
    }

    /// Number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns all events.
    pub fn events(&self) -> &[EventHandler] {
        &self.events
    }

    /// Iterator support.
    pub fn iter(&self) -> std::slice::Iter<'_, EventHandler> {
        self.events.iter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::{EventAction, EventKind};

    #[test]
    fn empty_collection() {
        let collection = EventCollection::new();

        assert!(collection.is_empty());
    }

    #[test]
    fn add_event() {
        let mut collection = EventCollection::new();

        collection.add(EventHandler::new(
            EventKind::Click,
            EventAction::new("login"),
        ));

        assert_eq!(collection.len(), 1,);
    }

    #[test]
    fn remove_event() {
        let mut collection = EventCollection::new();

        collection.add(EventHandler::new(
            EventKind::Click,
            EventAction::new("login"),
        ));

        let removed = collection.remove(0);

        assert!(removed.is_some());

        assert_eq!(collection.len(), 0,);
    }

    #[test]
    fn preserve_order() {
        let mut collection = EventCollection::new();

        collection.add(EventHandler::new(
            EventKind::Click,
            EventAction::new("login"),
        ));

        collection.add(EventHandler::new(
            EventKind::Submit,
            EventAction::new("submit"),
        ));

        let events = collection.events();

        assert_eq!(events[0].action().name(), "login",);

        assert_eq!(events[1].action().name(), "submit",);
    }
}

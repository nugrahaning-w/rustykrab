//! Event handler model.

use crate::event::{EventAction, EventKind, EventLike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventHandler {
    kind: EventKind,
    action: EventAction,
}

impl EventHandler {
    pub fn new(kind: EventKind, action: EventAction) -> Self {
        Self { kind, action }
    }

    pub fn kind(&self) -> &EventKind {
        &self.kind
    }

    pub fn action(&self) -> &EventAction {
        &self.action
    }
}

impl EventLike for EventHandler {
    fn kind(&self) -> &EventKind {
        &self.kind
    }

    fn action(&self) -> &EventAction {
        &self.action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_handler() {
        let handler = EventHandler::new(EventKind::Click, EventAction::new("login"));

        assert_eq!(handler.kind(), &EventKind::Click,);

        assert_eq!(handler.action().name(), "login",);
    }
}

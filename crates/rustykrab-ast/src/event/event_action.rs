//! Event action definition.

/// Represents an action triggered by an event.
///
/// Examples:
///
/// - login
/// - submit_form
/// - navigate_home
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventAction {
    name: String,
}

impl EventAction {
    /// Creates a new action.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns action name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

impl From<&str> for EventAction {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for EventAction {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_action() {
        let action = EventAction::new("login");

        assert_eq!(action.name(), "login",);
    }

    #[test]
    fn create_from_str() {
        let action: EventAction = "submit".into();

        assert_eq!(action.name(), "submit",);
    }
}

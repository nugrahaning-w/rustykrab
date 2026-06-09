use std::fmt;

/// Unique identifier for AST nodes.
///
/// NodeId is used for:
/// - Tree traversal
/// - Node lookup
/// - Validation
/// - Debugging
/// - Serialization
///
/// Example:
///
/// let id = NodeId::new("text_001");
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub String);

impl NodeId {
    /// Creates a new NodeId.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new("node")
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for NodeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node_id() {
        let id = NodeId::new("text_001");

        assert_eq!(id.as_str(), "text_001");
    }

    #[test]
    fn create_from_str() {
        let id: NodeId = "button_001".into();

        assert_eq!(id.as_str(), "button_001");
    }

    #[test]
    fn create_from_string() {
        let id: NodeId = String::from("image_001").into();

        assert_eq!(id.as_str(), "image_001");
    }

    #[test]
    fn display_node_id() {
        let id = NodeId::new("node_001");

        assert_eq!(id.to_string(), "node_001");
    }

    #[test]
    fn default_node_id() {
        let id = NodeId::default();

        assert_eq!(id.as_str(), "node");
    }

    #[test]
    fn equality() {
        let a = NodeId::new("node_1");

        let b = NodeId::new("node_1");

        assert_eq!(a, b);
    }
}

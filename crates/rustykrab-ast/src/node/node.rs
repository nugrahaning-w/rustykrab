use crate::{
    ids::NodeId,
    kinds::NodeKind,
    metadata::Metadata,
    property::{PropertyAccess, PropertyMap, PropertyValue},
};

/// Represents a single AST node.
///
/// Node is the fundamental building block
/// of the RustyKrab AST.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// Unique identifier.
    pub id: NodeId,

    /// Semantic node type.
    pub kind: NodeKind,

    /// Widget properties.
    pub properties: PropertyMap,

    /// Metadata.
    pub metadata: Metadata,

    /// Child nodes.
    pub children: Vec<Node>,
}

impl Node {
    /// Creates a new node.
    pub fn new(id: NodeId, kind: NodeKind) -> Self {
        Self {
            id,
            kind,
            properties: PropertyMap::default(),
            metadata: Metadata::default(),
            children: Vec::new(),
        }
    }

    /// Creates a node with metadata.
    pub fn with_metadata(id: NodeId, kind: NodeKind, metadata: Metadata) -> Self {
        Self {
            id,
            kind,
            properties: PropertyMap::default(),
            metadata,
            children: Vec::new(),
        }
    }

    /// Adds a child node.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Builder-style child insertion.
    pub fn child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    /// Builder-style property insertion.
    ///
    /// Example:
    ///
    /// ```rust
    /// use rustykrab_ast::prelude::*;
    ///
    /// let node = Node::new(
    ///     NodeId::new("text"),
    ///     NodeKind::Text,
    /// )
    /// .property(
    ///     PropertyKeys::VALUE,
    ///     PropertyValue::String(
    ///         "Hello".into(),
    ///     ),
    /// );
    /// ```
    pub fn property(mut self, key: impl Into<String>, value: PropertyValue) -> Self {
        self.properties.insert(key.into(), value);

        self
    }

    /// Removes a direct child.
    pub fn remove_child(&mut self, id: &NodeId) -> Option<Node> {
        let index = self.children.iter().position(|child| child.id == *id)?;

        Some(self.children.remove(index))
    }

    /// Returns direct child count.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns true if node has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Finds a direct child.
    pub fn find_child(&self, id: &NodeId) -> Option<&Node> {
        self.children.iter().find(|child| child.id == *id)
    }
}

impl PropertyAccess for Node {
    fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.properties.get(key)
    }

    fn set(&mut self, key: impl Into<String>, value: PropertyValue) {
        self.properties.insert(key.into(), value);
    }

    fn contains(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    fn remove(&mut self, key: &str) -> Option<PropertyValue> {
        self.properties.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node::new(NodeId::new("text_1"), NodeKind::Text);

        assert_eq!(node.kind, NodeKind::Text);

        assert!(node.properties.is_empty());
    }

    #[test]
    fn add_child() {
        let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

        root.add_child(Node::new(NodeId::new("text"), NodeKind::Text));

        assert_eq!(root.child_count(), 1);
    }

    #[test]
    fn remove_child() {
        let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

        root.add_child(Node::new(NodeId::new("text"), NodeKind::Text));

        let removed = root.remove_child(&NodeId::new("text"));

        assert!(removed.is_some());

        assert_eq!(root.child_count(), 0);
    }

    #[test]
    fn set_property() {
        let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

        node.set("value", PropertyValue::String("Hello".into()));

        assert!(node.contains("value"));
    }

    #[test]
    fn get_property() {
        let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

        node.set("value", PropertyValue::String("Hello".into()));

        assert_eq!(
            node.get("value"),
            Some(&PropertyValue::String("Hello".into(),),),
        );
    }

    #[test]
    fn remove_property() {
        let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

        node.set("value", PropertyValue::String("Hello".into()));

        let removed = node.remove("value");

        assert!(removed.is_some());

        assert!(!node.contains("value"));
    }

    #[test]
    fn builder_child() {
        let root = Node::new(NodeId::new("root"), NodeKind::VStack)
            .child(Node::new(NodeId::new("text"), NodeKind::Text));

        assert_eq!(root.child_count(), 1);
    }

    #[test]
    fn builder_property() {
        let node = Node::new(NodeId::new("text"), NodeKind::Text)
            .property("value", PropertyValue::String("Hello".into()));

        assert_eq!(
            node.get("value"),
            Some(&PropertyValue::String("Hello".into(),),),
        );
    }
    #[test]
    fn builder_multiple_properties() {
        let node = Node::new(NodeId::new("text"), NodeKind::Text)
            .property("value", PropertyValue::String("Hello".into()))
            .property("font_size", PropertyValue::Integer(16));

        assert_eq!(node.properties.len(), 2,);

        assert_eq!(node.get("font_size"), Some(&PropertyValue::Integer(16),),);
    }
    #[test]
    fn builder_tree_with_properties() {
        let root = Node::new(NodeId::new("root"), NodeKind::VStack).child(
            Node::new(NodeId::new("text"), NodeKind::Text)
                .property("value", PropertyValue::String("Hello".into())),
        );

        assert_eq!(root.child_count(), 1,);

        let child = root.find_child(&NodeId::new("text"));

        assert!(child.is_some());
    }
}

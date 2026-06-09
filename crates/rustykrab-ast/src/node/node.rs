use crate::{ids::NodeId, kinds::NodeKind, metadata::Metadata};

/// Represents a single AST node.
///
/// Node is the fundamental building block
/// of the RustyKrab AST.
///
/// Every widget, container, navigation element,
/// and future custom component is represented
/// by a Node.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// Unique identifier of the node.
    pub id: NodeId,

    /// Semantic type of the node.
    pub kind: NodeKind,

    /// Source mapping and compiler metadata.
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
            metadata: Metadata::default(),
            children: Vec::new(),
        }
    }

    /// Creates a new node with metadata.
    pub fn with_metadata(id: NodeId, kind: NodeKind, metadata: Metadata) -> Self {
        Self {
            id,
            kind,
            metadata,
            children: Vec::new(),
        }
    }

    /// Builder-style child insertion.
    pub fn child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    /// Adds a child node.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Removes a direct child by id.
    pub fn remove_child(&mut self, id: &NodeId) -> Option<Node> {
        let index = self.children.iter().position(|child| child.id == *id)?;

        Some(self.children.remove(index))
    }

    /// Returns the number of direct children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns true if the node has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Finds a direct child by id.
    pub fn find_child(&self, id: &NodeId) -> Option<&Node> {
        self.children.iter().find(|child| child.id == *id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_text_node() {
        let node = Node::new(NodeId::new("text_1"), NodeKind::Text);

        assert_eq!(node.id, NodeId::new("text_1"));

        assert_eq!(node.kind, NodeKind::Text);

        assert!(node.children.is_empty());
    }

    #[test]
    fn create_button_node() {
        let node = Node::new(NodeId::new("button_1"), NodeKind::Button);

        assert_eq!(node.kind, NodeKind::Button);
    }

    #[test]
    fn create_container_node() {
        let node = Node::new(NodeId::new("vstack_1"), NodeKind::VStack);

        assert!(node.kind.is_container());
    }

    #[test]
    fn create_node_with_metadata() {
        let metadata = Metadata::default();

        let node = Node::with_metadata(NodeId::new("text_1"), NodeKind::Text, metadata.clone());

        assert_eq!(node.metadata, metadata);
    }

    #[test]
    fn add_child() {
        let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

        let child = Node::new(NodeId::new("text_1"), NodeKind::Text);

        root.add_child(child);

        assert_eq!(root.child_count(), 1);

        assert!(root.has_children());
    }

    #[test]
    fn remove_child() {
        let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

        root.add_child(Node::new(NodeId::new("text_1"), NodeKind::Text));

        let removed = root.remove_child(&NodeId::new("text_1"));

        assert!(removed.is_some());

        assert_eq!(root.child_count(), 0);
    }

    #[test]
    fn find_child() {
        let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

        root.add_child(Node::new(NodeId::new("text_1"), NodeKind::Text));

        let child = root.find_child(&NodeId::new("text_1"));

        assert!(child.is_some());

        assert_eq!(child.unwrap().kind, NodeKind::Text);
    }

    #[test]
    fn has_children() {
        let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

        assert!(!root.has_children());

        root.add_child(Node::new(NodeId::new("text_1"), NodeKind::Text));

        assert!(root.has_children());
    }

    #[test]
    fn child_count() {
        let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

        root.add_child(Node::new(NodeId::new("text_1"), NodeKind::Text));

        root.add_child(Node::new(NodeId::new("button_1"), NodeKind::Button));

        assert_eq!(root.child_count(), 2);
    }
}

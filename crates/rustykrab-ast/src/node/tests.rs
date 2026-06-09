use crate::{ids::NodeId, kinds::NodeKind, node::Node};

#[test]
fn create_text_node() {
    let node = Node::new(NodeId::new("text"), NodeKind::Text);

    assert_eq!(node.kind, NodeKind::Text);
}

#[test]
fn create_button_node() {
    let node = Node::new(NodeId::new("button"), NodeKind::Button);

    assert_eq!(node.kind, NodeKind::Button);
}

#[test]
fn create_container_node() {
    let node = Node::new(NodeId::new("vstack"), NodeKind::VStack);

    assert!(node.kind.is_container());
}

#[test]
fn vstack_with_children() {
    let root = Node::new(NodeId::new("root"), NodeKind::VStack)
        .child(Node::new(NodeId::new("text"), NodeKind::Text))
        .child(Node::new(NodeId::new("button"), NodeKind::Button));

    assert_eq!(root.child_count(), 2);

    assert!(root.has_children());
}

#[test]
fn remove_child() {
    let mut root = Node::new(NodeId::new("root"), NodeKind::VStack);

    root.add_child(Node::new(NodeId::new("text"), NodeKind::Text));

    assert_eq!(root.child_count(), 1);

    root.remove_child(&NodeId::new("text"));

    assert_eq!(root.child_count(), 0);
}

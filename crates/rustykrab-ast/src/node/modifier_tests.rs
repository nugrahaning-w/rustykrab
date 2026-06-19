use crate::prelude::*;

#[test]
fn add_modifier_to_node() {
    let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

    node.add_modifier(Modifier::new(
        ModifierKind::Padding,
        ModifierValue::Integer(16),
    ));

    assert_eq!(node.modifier_count(), 1,);
}

#[test]
fn node_has_modifiers() {
    let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

    assert!(!node.has_modifiers());

    node.add_modifier(Modifier::new(
        ModifierKind::Padding,
        ModifierValue::Integer(16),
    ));

    assert!(node.has_modifiers());
}

#[test]
fn builder_modifier_api() {
    let node = Node::new(NodeId::new("text"), NodeKind::Text).padding(16);

    assert_eq!(node.modifier_count(), 1,);
}

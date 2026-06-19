use crate::prelude::*;

#[test]
fn prelude_exports_node() {
    let node = Node::new(NodeId::new("text"), NodeKind::Text);

    assert_eq!(node.kind, NodeKind::Text,);
}

#[test]
fn prelude_exports_property_system() {
    let node = Node::new(NodeId::new("text"), NodeKind::Text)
        .property(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

    assert!(node.get(PropertyKeys::VALUE,).is_some());
}

#[test]
fn prelude_exports_modifier_system() {
    let node = Node::new(NodeId::new("title"), NodeKind::Text)
        .padding(16)
        .background("#FF0000")
        .corner_radius(8)
        .opacity(0.8);

    assert_eq!(node.modifier_count(), 4,);
}

#[test]
fn prelude_exports_modifier_types() {
    let modifier = Modifier::new(ModifierKind::Padding, ModifierValue::Integer(16));

    assert_eq!(modifier.kind(), &ModifierKind::Padding,);
}

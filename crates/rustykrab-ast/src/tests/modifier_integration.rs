use crate::prelude::*;

#[test]
fn complex_modifier_chain() {
    let node = Node::new(NodeId::new("title"), NodeKind::Text)
        .padding(16)
        .background("#FF0000")
        .corner_radius(8)
        .opacity(0.8);

    assert_eq!(node.modifier_count(), 4,);
}

#[test]
fn modifier_order_is_preserved() {
    let node = Node::new(NodeId::new("title"), NodeKind::Text)
        .padding(16)
        .background("#FF0000")
        .opacity(0.8);

    let modifiers = node.modifiers.modifiers();

    assert_eq!(modifiers[0].kind(), &ModifierKind::Padding,);

    assert_eq!(modifiers[1].kind(), &ModifierKind::Background,);

    assert_eq!(modifiers[2].kind(), &ModifierKind::Opacity,);
}

#[test]
fn text_node_with_properties_and_modifiers() {
    let node = Node::new(NodeId::new("title"), NodeKind::Text)
        .property(
            PropertyKeys::VALUE,
            PropertyValue::String("Hello RustyKrab".into()),
        )
        .padding(16)
        .background("#FF0000");

    assert!(node.get(PropertyKeys::VALUE,).is_some());

    assert_eq!(node.modifier_count(), 2,);
}

use crate::{
    ids::NodeId,
    kinds::NodeKind,
    node::Node,
    property::{PropertyAccess, PropertyKeys, PropertyObject, PropertyValue},
};

#[test]
fn node_set_property() {
    let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

    node.set(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

    assert!(node.contains(PropertyKeys::VALUE));
}

#[test]
fn node_get_property() {
    let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

    node.set(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

    let value = node.get(PropertyKeys::VALUE);

    assert_eq!(value, Some(&PropertyValue::String("Hello".into(),),),);
}

#[test]
fn node_remove_property() {
    let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

    node.set(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

    let removed = node.remove(PropertyKeys::VALUE);

    assert!(removed.is_some());

    assert!(!node.contains(PropertyKeys::VALUE));
}

#[test]
fn node_replace_property() {
    let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

    node.set(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

    node.set(PropertyKeys::VALUE, PropertyValue::String("Welcome".into()));

    assert_eq!(
        node.get(PropertyKeys::VALUE),
        Some(&PropertyValue::String("Welcome".into(),),),
    );
}

#[test]
fn node_builder_property() {
    let node = Node::new(NodeId::new("text"), NodeKind::Text)
        .property(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

    assert_eq!(
        node.get(PropertyKeys::VALUE),
        Some(&PropertyValue::String("Hello".into(),),),
    );
}

#[test]
fn node_builder_multiple_properties() {
    let node = Node::new(NodeId::new("text"), NodeKind::Text)
        .property(PropertyKeys::VALUE, PropertyValue::String("Hello".into()))
        .property(PropertyKeys::FONT_SIZE, PropertyValue::Integer(16));

    assert_eq!(node.properties.len(), 2);

    assert_eq!(
        node.get(PropertyKeys::FONT_SIZE),
        Some(&PropertyValue::Integer(16),),
    );
}

#[test]
fn node_property_with_integer() {
    let mut node = Node::new(NodeId::new("text"), NodeKind::Text);

    node.set(PropertyKeys::FONT_SIZE, PropertyValue::Integer(18));

    assert_eq!(
        node.get(PropertyKeys::FONT_SIZE),
        Some(&PropertyValue::Integer(18),),
    );
}

#[test]
fn node_property_with_boolean() {
    let mut node = Node::new(NodeId::new("button"), NodeKind::Button);

    node.set("enabled", PropertyValue::Boolean(true));

    assert_eq!(node.get("enabled"), Some(&PropertyValue::Boolean(true),),);
}

#[test]
fn node_property_missing() {
    let node = Node::new(NodeId::new("text"), NodeKind::Text);

    assert!(node.get(PropertyKeys::VALUE).is_none());
}
#[test]
fn node_with_complex_property() {
    let mut shadow = PropertyObject::new();

    shadow.insert("radius".into(), PropertyValue::Integer(8));

    let node = Node::new(NodeId::new("text"), NodeKind::Text)
        .property("shadow", PropertyValue::Object(shadow));

    assert!(node.get("shadow").is_some());
}
#[test]
fn node_with_array_property() {
    let node = Node::new(NodeId::new("list"), NodeKind::List).property(
        "items",
        PropertyValue::Array(vec![
            PropertyValue::String("A".into()),
            PropertyValue::String("B".into()),
        ]),
    );

    assert!(node.get("items").is_some());
}

use crate::property::{
    PropertyMap, PropertyObject, PropertyValue, has_property, require_property,
    require_string_property,
};

#[test]
fn create_property_map() {
    let properties = PropertyMap::new();

    assert!(properties.is_empty());
}

#[test]
fn insert_property() {
    let mut properties = PropertyMap::new();

    properties.insert("value".into(), PropertyValue::String("Hello".into()));

    assert_eq!(properties.len(), 1);
}

#[test]
fn get_property() {
    let mut properties = PropertyMap::new();

    properties.insert("value".into(), PropertyValue::String("Hello".into()));

    let value = properties.get("value");

    assert_eq!(value, Some(&PropertyValue::String("Hello".into(),),),);
}

#[test]
fn contains_property() {
    let mut properties = PropertyMap::new();

    properties.insert("value".into(), PropertyValue::String("Hello".into()));

    assert!(properties.contains_key("value"));
}

#[test]
fn remove_property() {
    let mut properties = PropertyMap::new();

    properties.insert("value".into(), PropertyValue::String("Hello".into()));

    let removed = properties.remove("value");

    assert!(removed.is_some());

    assert!(properties.is_empty());
}

#[test]
fn create_property_object() {
    let mut object = PropertyObject::new();

    object.insert("radius".into(), PropertyValue::Integer(8));

    assert_eq!(object.len(), 1);
}

#[test]
fn array_property() {
    let value = PropertyValue::Array(vec![
        PropertyValue::String("A".into()),
        PropertyValue::String("B".into()),
    ]);

    assert!(value.is_array());
}

#[test]
fn object_property() {
    let mut object = PropertyObject::new();

    object.insert("radius".into(), PropertyValue::Integer(8));

    let value = PropertyValue::Object(object);

    assert!(value.is_object());
}

#[test]
fn validation_has_property() {
    let mut properties = PropertyMap::new();

    properties.insert("value".into(), PropertyValue::String("Hello".into()));

    assert!(has_property(&properties, "value"));
}

#[test]
fn validation_require_property() {
    let mut properties = PropertyMap::new();

    properties.insert("value".into(), PropertyValue::String("Hello".into()));

    let value = require_property(&properties, "value");

    assert!(value.is_ok());
}

#[test]
fn validation_require_string_property() {
    let mut properties = PropertyMap::new();

    properties.insert("value".into(), PropertyValue::String("Hello".into()));

    let value = require_string_property(&properties, "value");

    assert_eq!(value.unwrap(), "Hello");
}

#[test]
fn validation_missing_property() {
    let properties = PropertyMap::new();

    let value = require_property(&properties, "value");

    assert!(value.is_err());
}
#[test]
fn nested_object_property() {
    let mut shadow = PropertyObject::new();

    shadow.insert("radius".into(), PropertyValue::Integer(8));

    shadow.insert("opacity".into(), PropertyValue::Float(0.3));

    let value = PropertyValue::Object(shadow);

    assert!(value.is_object());
}
#[test]
fn deep_nested_object() {
    let mut shadow = PropertyObject::new();

    shadow.insert("radius".into(), PropertyValue::Integer(8));

    let mut style = PropertyObject::new();

    style.insert("shadow".into(), PropertyValue::Object(shadow));

    let value = PropertyValue::Object(style);

    assert!(value.is_object());
}
#[test]
fn string_array_property() {
    let value = PropertyValue::Array(vec![
        PropertyValue::String("A".into()),
        PropertyValue::String("B".into()),
        PropertyValue::String("C".into()),
    ]);

    assert!(value.is_array());
}
#[test]
fn integer_array_property() {
    let value = PropertyValue::Array(vec![
        PropertyValue::Integer(1),
        PropertyValue::Integer(2),
        PropertyValue::Integer(3),
    ]);

    assert!(value.is_array());
}
#[test]
fn array_of_objects() {
    let mut item1 = PropertyObject::new();

    item1.insert("title".into(), PropertyValue::String("A".into()));

    let mut item2 = PropertyObject::new();

    item2.insert("title".into(), PropertyValue::String("B".into()));

    let value = PropertyValue::Array(vec![
        PropertyValue::Object(item1),
        PropertyValue::Object(item2),
    ]);

    assert!(value.is_array());
}
#[test]
fn mixed_property_structure() {
    let mut properties = PropertyMap::new();

    properties.insert("title".into(), PropertyValue::String("Hello".into()));

    properties.insert("enabled".into(), PropertyValue::Boolean(true));

    properties.insert("font_size".into(), PropertyValue::Integer(16));

    let mut shadow = PropertyObject::new();

    shadow.insert("radius".into(), PropertyValue::Integer(8));

    properties.insert("shadow".into(), PropertyValue::Object(shadow));

    assert_eq!(properties.len(), 4,);
}
#[test]
fn empty_array_property() {
    let value = PropertyValue::Array(Vec::new());

    assert!(value.is_array());
}
#[test]
fn empty_object_property() {
    let value = PropertyValue::Object(PropertyObject::new());

    assert!(value.is_object());
}

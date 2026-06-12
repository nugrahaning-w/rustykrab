use std::collections::HashMap;

use super::PropertyValue;

/// Stores properties associated with a Node.
///
/// PropertyMap is the primary storage mechanism
/// for widget configuration in the RustyKrab AST.
///
/// Example:
///
/// {
///     "value": String("Hello"),
///     "font_size": Integer(16),
///     "color": Color("#FF0000")
/// }
///
pub type PropertyMap = HashMap<String, PropertyValue>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_property_map() {
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

        assert_eq!(
            properties.get("value"),
            Some(&PropertyValue::String("Hello".into(),),),
        );
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
}

use std::collections::HashMap;

use super::PropertyValue;

/// Represents a structured property object.
///
/// PropertyObject is used to store nested property values.
///
/// Example:
///
/// {
///   "shadow": {
///     "radius": 8,
///     "opacity": 0.3
///   }
/// }
///
/// Internally:
///
/// PropertyObject {
///     "radius" => Integer(8),
///     "opacity" => Float(0.3),
/// }
///
pub type PropertyObject = HashMap<String, PropertyValue>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_property_object() {
        let object = PropertyObject::new();

        assert!(object.is_empty());
    }

    #[test]
    fn insert_property_into_object() {
        let mut object = PropertyObject::new();

        object.insert("radius".into(), PropertyValue::Integer(8));

        assert_eq!(object.len(), 1);

        assert_eq!(object.get("radius"), Some(&PropertyValue::Integer(8)));
    }

    #[test]
    fn nested_object_properties() {
        let mut shadow = PropertyObject::new();

        shadow.insert("radius".into(), PropertyValue::Integer(8));

        shadow.insert("opacity".into(), PropertyValue::Float(0.3));

        assert_eq!(shadow.len(), 2);
    }
}

use super::{PropertyMap, PropertyValue};

/// Returns true if a property exists.
pub fn has_property(properties: &PropertyMap, key: &str) -> bool {
    properties.contains_key(key)
}

/// Returns a property if it exists.
pub fn require_property<'a>(
    properties: &'a PropertyMap,
    key: &str,
) -> Result<&'a PropertyValue, String> {
    properties
        .get(key)
        .ok_or_else(|| format!("Required property '{}' is missing", key))
}

/// Ensures the property exists and is a String.
pub fn require_string_property<'a>(
    properties: &'a PropertyMap,
    key: &str,
) -> Result<&'a String, String> {
    match require_property(properties, key)? {
        PropertyValue::String(value) => Ok(value),

        _ => Err(format!("Property '{}' must be a String", key)),
    }
}

/// Ensures the property exists and is an Integer.
pub fn require_integer_property(properties: &PropertyMap, key: &str) -> Result<i64, String> {
    match require_property(properties, key)? {
        PropertyValue::Integer(value) => Ok(*value),

        _ => Err(format!("Property '{}' must be an Integer", key)),
    }
}

/// Ensures the property exists and is a Boolean.
pub fn require_boolean_property(properties: &PropertyMap, key: &str) -> Result<bool, String> {
    match require_property(properties, key)? {
        PropertyValue::Boolean(value) => Ok(*value),

        _ => Err(format!("Property '{}' must be a Boolean", key)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_existing_property() {
        let mut props = PropertyMap::new();

        props.insert("value".into(), PropertyValue::String("Hello".into()));

        assert!(has_property(&props, "value"));
    }

    #[test]
    fn has_missing_property() {
        let props = PropertyMap::new();

        assert!(!has_property(&props, "value"));
    }

    #[test]
    fn require_existing_property() {
        let mut props = PropertyMap::new();

        props.insert("value".into(), PropertyValue::String("Hello".into()));

        let value = require_property(&props, "value");

        assert!(value.is_ok());
    }

    #[test]
    fn require_missing_property() {
        let props = PropertyMap::new();

        let value = require_property(&props, "value");

        assert!(value.is_err());
    }

    #[test]
    fn require_string_success() {
        let mut props = PropertyMap::new();

        props.insert("value".into(), PropertyValue::String("Hello".into()));

        let value = require_string_property(&props, "value");

        assert_eq!(value.unwrap(), "Hello");
    }

    #[test]
    fn require_integer_success() {
        let mut props = PropertyMap::new();

        props.insert("font_size".into(), PropertyValue::Integer(16));

        let value = require_integer_property(&props, "font_size");

        assert_eq!(value.unwrap(), 16);
    }

    #[test]
    fn require_boolean_success() {
        let mut props = PropertyMap::new();

        props.insert("enabled".into(), PropertyValue::Boolean(true));

        let value = require_boolean_property(&props, "enabled");

        assert!(value.unwrap());
    }

    #[test]
    fn wrong_property_type() {
        let mut props = PropertyMap::new();

        props.insert("font_size".into(), PropertyValue::String("large".into()));

        let value = require_integer_property(&props, "font_size");

        assert!(value.is_err());
    }
}

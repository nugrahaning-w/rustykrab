use super::PropertyObject;

/// Represents a property value stored in a PropertyMap.
///
/// PropertyValue is the universal value container
/// used throughout the RustyKrab AST.
///
/// It supports:
/// - Primitive values
/// - Structured objects
/// - Arrays
/// - References
/// - Expressions
/// - Future runtime integrations
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// String value.
    String(String),

    /// Integer value.
    Integer(i64),

    /// Floating point value.
    Float(f64),

    /// Boolean value.
    Boolean(bool),

    /// Enum-like string value.
    Enum(String),

    /// Color representation.
    ///
    /// Examples:
    ///
    /// "#FF0000"
    /// "primary"
    Color(String),

    /// Runtime expression.
    ///
    /// Example:
    ///
    /// "counter + 1"
    Expression(String),

    /// Reference to another object.
    ///
    /// Example:
    ///
    /// "counter"
    Reference(String),

    /// Function or action reference.
    ///
    /// Example:
    ///
    /// "login"
    Function(String),

    /// Array value.
    Array(Vec<PropertyValue>),

    /// Nested object value.
    Object(PropertyObject),

    /// Null value.
    Null,
}

impl PropertyValue {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }
}

impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for PropertyValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<i64> for PropertyValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for PropertyValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for PropertyValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_property() {
        let value = PropertyValue::String("Hello".into());

        assert!(value.is_string());
    }

    #[test]
    fn integer_property() {
        let value = PropertyValue::Integer(42);

        assert!(value.is_integer());
    }

    #[test]
    fn float_property() {
        let value = PropertyValue::Float(16.5);

        assert!(value.is_float());
    }

    #[test]
    fn boolean_property() {
        let value = PropertyValue::Boolean(true);

        assert!(value.is_boolean());
    }

    #[test]
    fn null_property() {
        let value = PropertyValue::Null;

        assert!(value.is_null());
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
    fn from_str() {
        let value: PropertyValue = "Hello".into();

        assert_eq!(value, PropertyValue::String("Hello".into()));
    }

    #[test]
    fn from_i64() {
        let value: PropertyValue = 10_i64.into();

        assert_eq!(value, PropertyValue::Integer(10));
    }

    #[test]
    fn from_f64() {
        let value: PropertyValue = 1.5_f64.into();

        assert_eq!(value, PropertyValue::Float(1.5,),);
    }

    #[test]
    fn from_bool() {
        let value: PropertyValue = true.into();

        assert_eq!(value, PropertyValue::Boolean(true));
    }
}

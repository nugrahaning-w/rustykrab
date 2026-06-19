use std::collections::HashMap;

/// Represents a modifier value.
///
/// ModifierValue stores platform-independent
/// layout and styling information.
///
/// Examples:
///
/// Padding(16)
/// Background("#FF0000")
/// Opacity(0.8)
///
#[derive(Debug, Clone, PartialEq)]
pub enum ModifierValue {
    /// String value.
    String(String),

    /// Integer value.
    Integer(i64),

    /// Floating-point value.
    Float(f64),

    /// Boolean value.
    Boolean(bool),

    /// Color representation.
    ///
    /// Examples:
    ///
    /// "#FF0000"
    /// "primary"
    Color(String),

    /// Array value.
    Array(Vec<ModifierValue>),

    /// Object value.
    Object(HashMap<String, ModifierValue>),

    /// Null value.
    Null,
}

impl ModifierValue {
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

    pub fn is_color(&self) -> bool {
        matches!(self, Self::Color(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }
}

impl From<String> for ModifierValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ModifierValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<i64> for ModifierValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for ModifierValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for ModifierValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_value() {
        let value = ModifierValue::String("hello".into());

        assert!(value.is_string());
    }

    #[test]
    fn integer_value() {
        let value = ModifierValue::Integer(16);

        assert!(value.is_integer());
    }

    #[test]
    fn float_value() {
        let value = ModifierValue::Float(0.8);

        assert!(value.is_float());
    }

    #[test]
    fn boolean_value() {
        let value = ModifierValue::Boolean(true);

        assert!(value.is_boolean());
    }

    #[test]
    fn color_value() {
        let value = ModifierValue::Color("#FF0000".into());

        assert!(value.is_color());
    }

    #[test]
    fn null_value() {
        let value = ModifierValue::Null;

        assert!(value.is_null());
    }

    #[test]
    fn array_value() {
        let value =
            ModifierValue::Array(vec![ModifierValue::Integer(1), ModifierValue::Integer(2)]);

        assert!(value.is_array());
    }

    #[test]
    fn object_value() {
        let mut object = HashMap::new();

        object.insert("radius".into(), ModifierValue::Integer(8));

        let value = ModifierValue::Object(object);

        assert!(value.is_object());
    }

    #[test]
    fn from_string() {
        let value: ModifierValue = "hello".into();

        assert_eq!(value, ModifierValue::String("hello".into(),),);
    }

    #[test]
    fn from_integer() {
        let value: ModifierValue = 16_i64.into();

        assert_eq!(value, ModifierValue::Integer(16,),);
    }

    #[test]
    fn from_float() {
        let value: ModifierValue = 0.8_f64.into();

        assert_eq!(value, ModifierValue::Float(0.8,),);
    }

    #[test]
    fn from_bool() {
        let value: ModifierValue = true.into();

        assert_eq!(value, ModifierValue::Boolean(true,),);
    }
}

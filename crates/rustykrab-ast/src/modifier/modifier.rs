use crate::modifier::{ModifierKind, ModifierValue};

/// Represents a single modifier.
///
/// A Modifier combines:
///
/// - ModifierKind
/// - ModifierValue
///
/// Example:
///
/// Modifier {
///     kind: ModifierKind::Padding,
///     value: ModifierValue::Integer(16),
/// }
///
#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub kind: ModifierKind,

    pub value: ModifierValue,
}

impl Modifier {
    /// Creates a new modifier.
    pub fn new(kind: ModifierKind, value: ModifierValue) -> Self {
        Self { kind, value }
    }

    /// Returns modifier kind.
    pub fn kind(&self) -> &ModifierKind {
        &self.kind
    }

    /// Returns modifier value.
    pub fn value(&self) -> &ModifierValue {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_padding_modifier() {
        let modifier = Modifier::new(ModifierKind::Padding, ModifierValue::Integer(16));

        assert_eq!(modifier.kind, ModifierKind::Padding,);

        assert_eq!(modifier.value, ModifierValue::Integer(16,),);
    }

    #[test]
    fn create_background_modifier() {
        let modifier = Modifier::new(
            ModifierKind::Background,
            ModifierValue::Color("#FF0000".into()),
        );

        assert_eq!(modifier.kind, ModifierKind::Background,);
    }

    #[test]
    fn kind_accessor() {
        let modifier = Modifier::new(ModifierKind::Opacity, ModifierValue::Float(0.8));

        assert_eq!(modifier.kind(), &ModifierKind::Opacity,);
    }

    #[test]
    fn value_accessor() {
        let modifier = Modifier::new(ModifierKind::Opacity, ModifierValue::Float(0.8));

        assert_eq!(modifier.value(), &ModifierValue::Float(0.8,),);
    }

    #[test]
    fn custom_modifier() {
        let modifier = Modifier::new(
            ModifierKind::Custom("glass".into()),
            ModifierValue::Boolean(true),
        );

        assert!(matches!(modifier.kind, ModifierKind::Custom(_)));
    }
}

use crate::modifier::{Modifier, ModifierKind, ModifierValue};

/// Common interface for all modifier types.
///
/// This trait allows future modifier implementations
/// to be handled generically by:
///
/// - Validators
/// - Generators
/// - Visitors
/// - Plugin Systems
pub trait ModifierLike {
    /// Returns modifier kind.
    fn kind(&self) -> &ModifierKind;

    /// Returns modifier value.
    fn value(&self) -> &ModifierValue;

    /// Returns true if this modifier is layout-related.
    fn is_layout(&self) -> bool {
        self.kind().is_layout()
    }

    /// Returns true if this modifier is style-related.
    fn is_style(&self) -> bool {
        self.kind().is_style()
    }

    /// Returns true if this modifier is visual-effect-related.
    fn is_visual_effect(&self) -> bool {
        self.kind().is_visual_effect()
    }

    /// Returns true if this modifier is custom.
    fn is_custom(&self) -> bool {
        self.kind().is_custom()
    }
}

impl ModifierLike for Modifier {
    fn kind(&self) -> &ModifierKind {
        &self.kind
    }

    fn value(&self) -> &ModifierValue {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_modifier_trait() {
        let modifier = Modifier::new(ModifierKind::Padding, ModifierValue::Integer(16));

        assert!(modifier.is_layout());
    }

    #[test]
    fn style_modifier_trait() {
        let modifier = Modifier::new(
            ModifierKind::Background,
            ModifierValue::Color("#FF0000".into()),
        );

        assert!(modifier.is_style());
    }

    #[test]
    fn visual_modifier_trait() {
        let modifier = Modifier::new(ModifierKind::Opacity, ModifierValue::Float(0.8));

        assert!(modifier.is_visual_effect());
    }

    #[test]
    fn custom_modifier_trait() {
        let modifier = Modifier::new(
            ModifierKind::Custom("glass".into()),
            ModifierValue::Boolean(true),
        );

        assert!(modifier.is_custom());
    }
}

/// High-level modifier categories.
///
/// Categories help:
///
/// - Validation
/// - Code Generation
/// - Modifier Organization
/// - Future Plugin Systems
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierCategory {
    /// Layout and sizing.
    Layout,

    /// Visual styling.
    Style,

    /// Visual effects.
    VisualEffect,

    /// User interaction.
    Interaction,

    /// Accessibility.
    Accessibility,

    /// Animations.
    Animation,
}

impl ModifierCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Layout => "layout",
            Self::Style => "style",
            Self::VisualEffect => "visual_effect",
            Self::Interaction => "interaction",
            Self::Accessibility => "accessibility",
            Self::Animation => "animation",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_name() {
        assert_eq!(ModifierCategory::Layout.as_str(), "layout",);
    }
}

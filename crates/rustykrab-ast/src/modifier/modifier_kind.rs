/// Defines all supported modifier types.
///
/// ModifierKind represents platform-independent
/// UI styling and layout operations.
///
/// These modifiers can later be translated into:
///
/// - SwiftUI Modifiers
/// - Jetpack Compose Modifiers
/// - Web CSS Styles
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModifierKind {
    // ============================================================
    // Layout
    // ============================================================
    Padding,

    Margin,

    Width,

    Height,

    MinWidth,

    MinHeight,

    MaxWidth,

    MaxHeight,

    Frame,

    Alignment,

    // ============================================================
    // Style
    // ============================================================
    Background,

    ForegroundColor,

    Border,

    CornerRadius,

    // ============================================================
    // Visual Effects
    // ============================================================
    Opacity,

    Shadow,

    Blur,

    // ============================================================
    // Interaction
    // ============================================================
    Disabled,

    Clickable,

    Focusable,

    // ============================================================
    // Accessibility
    // ============================================================
    AccessibilityLabel,

    AccessibilityIdentifier,

    // ============================================================
    // Animation
    // ============================================================
    Animation,

    Transition,

    // ============================================================
    // Custom
    // ============================================================
    Custom(String),
}

impl ModifierKind {
    /// Returns true if this modifier affects layout.
    pub fn is_layout(&self) -> bool {
        matches!(
            self,
            Self::Padding
                | Self::Margin
                | Self::Width
                | Self::Height
                | Self::MinWidth
                | Self::MinHeight
                | Self::MaxWidth
                | Self::MaxHeight
                | Self::Frame
                | Self::Alignment
        )
    }

    /// Returns true if this modifier affects styling.
    pub fn is_style(&self) -> bool {
        matches!(
            self,
            Self::Background | Self::ForegroundColor | Self::Border | Self::CornerRadius
        )
    }

    /// Returns true if this modifier affects visual effects.
    pub fn is_visual_effect(&self) -> bool {
        matches!(self, Self::Opacity | Self::Shadow | Self::Blur)
    }

    /// Returns true if this modifier is custom.
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_modifier() {
        assert!(ModifierKind::Padding.is_layout());
    }

    #[test]
    fn style_modifier() {
        assert!(ModifierKind::Background.is_style());
    }

    #[test]
    fn visual_modifier() {
        assert!(ModifierKind::Opacity.is_visual_effect());
    }

    #[test]
    fn custom_modifier() {
        assert!(ModifierKind::Custom("glass".into()).is_custom());
    }
}

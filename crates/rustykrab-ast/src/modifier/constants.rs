/// Standard modifier names.
///
/// Used by:
///
/// - Serialization
/// - Validation
/// - Generator Mapping
///
pub struct ModifierNames;

impl ModifierNames {
    // ============================================================
    // Layout
    // ============================================================

    pub const PADDING: &'static str = "padding";

    pub const MARGIN: &'static str = "margin";

    pub const WIDTH: &'static str = "width";

    pub const HEIGHT: &'static str = "height";

    pub const FRAME: &'static str = "frame";

    pub const ALIGNMENT: &'static str = "alignment";

    // ============================================================
    // Style
    // ============================================================

    pub const BACKGROUND: &'static str = "background";

    pub const FOREGROUND_COLOR: &'static str = "foreground_color";

    pub const BORDER: &'static str = "border";

    pub const CORNER_RADIUS: &'static str = "corner_radius";

    // ============================================================
    // Effects
    // ============================================================

    pub const OPACITY: &'static str = "opacity";

    pub const SHADOW: &'static str = "shadow";

    pub const BLUR: &'static str = "blur";

    // ============================================================
    // Accessibility
    // ============================================================

    pub const ACCESSIBILITY_LABEL: &'static str = "accessibility_label";

    pub const ACCESSIBILITY_IDENTIFIER: &'static str = "accessibility_identifier";

    // ============================================================
    // Animation
    // ============================================================

    pub const ANIMATION: &'static str = "animation";

    pub const TRANSITION: &'static str = "transition";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_exist() {
        assert_eq!(ModifierNames::PADDING, "padding",);

        assert_eq!(ModifierNames::OPACITY, "opacity",);
    }
}

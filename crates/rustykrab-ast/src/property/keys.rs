/// Standard property keys used throughout
/// the RustyKrab AST.
///
/// Using constants prevents typos and
/// provides consistency between:
///
/// - Widget DSL
/// - AST
/// - Validators
/// - Generators
///
/// Example:
///
/// node.set(
///     PropertyKeys::VALUE,
///     PropertyValue::String(
///         "Hello".into()
///     ),
/// );
///
pub struct PropertyKeys;

impl PropertyKeys {
    // ============================================================
    // Common
    // ============================================================

    pub const VALUE: &str = "value";

    pub const TITLE: &str = "title";

    pub const LABEL: &str = "label";

    pub const NAME: &str = "name";

    pub const ID: &str = "id";

    // ============================================================
    // Text
    // ============================================================

    pub const PLACEHOLDER: &str = "placeholder";

    pub const TEXT: &str = "text";

    pub const FONT_SIZE: &str = "font_size";

    pub const FONT_WEIGHT: &str = "font_weight";

    pub const FONT_FAMILY: &str = "font_family";

    pub const COLOR: &str = "color";

    // ============================================================
    // Image
    // ============================================================

    pub const SOURCE: &str = "source";

    pub const ALT_TEXT: &str = "alt_text";

    pub const CONTENT_MODE: &str = "content_mode";

    // ============================================================
    // Layout
    // ============================================================

    pub const WIDTH: &str = "width";

    pub const HEIGHT: &str = "height";

    pub const MIN_WIDTH: &str = "min_width";

    pub const MIN_HEIGHT: &str = "min_height";

    pub const MAX_WIDTH: &str = "max_width";

    pub const MAX_HEIGHT: &str = "max_height";

    pub const SPACING: &str = "spacing";

    pub const ALIGNMENT: &str = "alignment";

    // ============================================================
    // State
    // ============================================================

    pub const STATE: &str = "state";

    pub const BINDING: &str = "binding";

    pub const REFERENCE: &str = "reference";

    // ============================================================
    // Navigation
    // ============================================================

    pub const DESTINATION: &str = "destination";

    pub const ROUTE: &str = "route";

    // ============================================================
    // Collection
    // ============================================================

    pub const ITEMS: &str = "items";

    pub const DATA_SOURCE: &str = "data_source";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_key() {
        assert_eq!(PropertyKeys::VALUE, "value");
    }

    #[test]
    fn title_key() {
        assert_eq!(PropertyKeys::TITLE, "title");
    }

    #[test]
    fn source_key() {
        assert_eq!(PropertyKeys::SOURCE, "source");
    }

    #[test]
    fn width_key() {
        assert_eq!(PropertyKeys::WIDTH, "width");
    }
}

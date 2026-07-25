//! Event kinds supported by the RustyKrab AST.

/// Represents a platform-independent UI event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventKind {
    // ============================================================
    // Pointer / Gesture Events
    // ============================================================
    Click,

    Tap,

    DoubleTap,

    LongPress,

    // ============================================================
    // Focus Events
    // ============================================================
    Focus,

    Blur,

    // ============================================================
    // Input Events
    // ============================================================
    Change,

    Submit,

    // ============================================================
    // Lifecycle Events
    // ============================================================
    Appear,

    Disappear,

    // ============================================================
    // Custom Event
    // ============================================================
    Custom(String),
}

impl EventKind {
    /// Returns a string representation of the event.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Click => "click",
            Self::Tap => "tap",
            Self::DoubleTap => "double_tap",
            Self::LongPress => "long_press",
            Self::Focus => "focus",
            Self::Blur => "blur",
            Self::Change => "change",
            Self::Submit => "submit",
            Self::Appear => "appear",
            Self::Disappear => "disappear",
            Self::Custom(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn click_name() {
        assert_eq!(EventKind::Click.as_str(), "click",);
    }

    #[test]
    fn custom_name() {
        let kind = EventKind::Custom("refresh".into());

        assert_eq!(kind.as_str(), "refresh",);
    }
}

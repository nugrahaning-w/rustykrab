/// Represents the version of the RustyKrab AST schema.
///
/// Used to:
/// - Track AST structure changes
/// - Ensure generator compatibility
/// - Support future migration tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AstVersion {
    pub major: u16,
    pub minor: u16,
}

impl AstVersion {
    /// Creates a new AST version.
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Returns the current supported AST version.
    pub const fn current() -> Self {
        Self { major: 1, minor: 0 }
    }
}

impl Default for AstVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl std::fmt::Display for AstVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_version() {
        let version = AstVersion::new(1, 0);

        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
    }

    #[test]
    fn current_version() {
        let version = AstVersion::current();

        assert_eq!(version, AstVersion::new(1, 0));
    }

    #[test]
    fn default_version() {
        let version = AstVersion::default();

        assert_eq!(version, AstVersion::new(1, 0));
    }

    #[test]
    fn display_version() {
        let version = AstVersion::new(1, 0);

        assert_eq!(version.to_string(), "1.0");
    }

    #[test]
    fn compare_versions() {
        let v1 = AstVersion::new(1, 0);

        let v2 = AstVersion::new(1, 1);

        assert!(v2 > v1);
    }
}

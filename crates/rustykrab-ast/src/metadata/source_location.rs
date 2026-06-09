#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: String,

    pub line: usize,

    pub column: usize,
}

impl SourceLocation {
    pub fn new(file: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            file: file.into(),
            line,
            column,
        }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            file: String::new(),
            line: 1,
            column: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_source_location() {
        let source = SourceLocation::new("main.rs", 10, 5);

        assert_eq!(source.file, "main.rs");

        assert_eq!(source.line, 10);

        assert_eq!(source.column, 5);
    }
}

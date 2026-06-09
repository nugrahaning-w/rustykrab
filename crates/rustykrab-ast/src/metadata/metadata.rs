use super::{CompilerMetadata, DiagnosticTag, SourceLocation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub source: SourceLocation,

    pub compiler: CompilerMetadata,

    pub diagnostics: Vec<DiagnosticTag>,
}

impl Metadata {
    pub fn new(source: SourceLocation) -> Self {
        Self {
            source,
            compiler: CompilerMetadata::default(),
            diagnostics: Vec::new(),
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            source: SourceLocation::default(),
            compiler: CompilerMetadata::default(),
            diagnostics: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_metadata() {
        let source = SourceLocation::new("main.rs", 1, 1);

        let metadata = Metadata::new(source);

        assert_eq!(metadata.source.file, "main.rs");

        assert!(metadata.diagnostics.is_empty());
    }
}

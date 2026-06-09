use crate::metadata::{CompilerMetadata, Metadata, SourceLocation};

#[test]
fn create_source_location() {
    let source = SourceLocation::new("main.rs", 10, 5);

    assert_eq!(source.file, "main.rs");
}

#[test]
fn create_compiler_metadata() {
    let compiler = CompilerMetadata::new("0.1.0", "1.0");

    assert_eq!(compiler.compiler_version, "0.1.0");
}

#[test]
fn create_metadata() {
    let metadata = Metadata::default();

    assert!(metadata.diagnostics.is_empty());
}

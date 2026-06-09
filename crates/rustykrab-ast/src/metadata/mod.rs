mod compiler_metadata;
mod diagnostic_tag;
mod metadata;
mod source_location;

pub use compiler_metadata::CompilerMetadata;
pub use diagnostic_tag::DiagnosticTag;
pub use metadata::Metadata;
pub use source_location::SourceLocation;

#[cfg(test)]
mod tests;

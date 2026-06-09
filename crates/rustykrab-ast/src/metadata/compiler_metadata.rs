#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerMetadata {
    pub compiler_version: String,

    pub ast_version: String,
}

impl CompilerMetadata {
    pub fn new(compiler_version: impl Into<String>, ast_version: impl Into<String>) -> Self {
        Self {
            compiler_version: compiler_version.into(),
            ast_version: ast_version.into(),
        }
    }
}

impl Default for CompilerMetadata {
    fn default() -> Self {
        Self {
            compiler_version: "0.1.0".into(),
            ast_version: "1.0".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_compiler_metadata() {
        let metadata = CompilerMetadata::new("0.1.0", "1.0");

        assert_eq!(metadata.compiler_version, "0.1.0");

        assert_eq!(metadata.ast_version, "1.0");
    }
}

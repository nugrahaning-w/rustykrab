use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustyKrabError {

    #[error("Parsing error: {0}")]
    Parsing(String),

    #[error("AST error: {0}")]
    Ast(String),

    #[error("Generator error: {0}")]
    Generator(String),

    #[error("CLI error: {0}")]
    Cli(String),

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_parsing_error() {

        let error =
            RustyKrabError::Parsing(
                "invalid widget".into()
            );

        assert!(
            error.to_string()
                .contains("Parsing error")
        );
    }
}

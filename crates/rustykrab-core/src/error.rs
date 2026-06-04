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

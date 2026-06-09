#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticTag {
    Warning(String),

    Error(String),

    Hint(String),

    Deprecated(String),
}

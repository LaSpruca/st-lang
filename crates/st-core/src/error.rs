#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[error("{row}:{column}: {error}")]
pub struct CompileError {
    row: usize,
    column: usize,
    error: CompileErrorEnum,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum CompileErrorEnum {
    #[error("Unclosed string")]
    UnclosedString,

    #[error("Expected ''")]
    ExpectedPipe(String),
}

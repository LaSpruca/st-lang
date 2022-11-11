use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{file}({line}:{column}) Found unclosed string {content}")]
    UnclosedString {
        file: String,
        line: usize,
        column: usize,
        content: String,
    },
}

pub type Result<T> = core::result::Result<T, Error>;

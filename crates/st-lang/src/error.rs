use std::num::{ParseFloatError, ParseIntError};

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

    #[error("{file}({line}:{column}) Could not parse {content} as int due to {parse_error}")]
    IntParseError {
        file: String,
        line: usize,
        column: usize,
        content: String,
        parse_error: ParseIntError,
    },

    #[error("{file}({line}:{column}) Could not parse {content} as int due to {parse_error}")]
    FloatParseError {
        file: String,
        line: usize,
        column: usize,
        content: String,
        parse_error: ParseFloatError,
    },
}

pub type Result<T> = core::result::Result<T, Error>;

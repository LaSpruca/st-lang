mod error;
pub mod symbols;
pub mod tokenizer;
pub use error::{Error, Result};

#[derive(Default)]
pub struct Span {
    line: usize,
    row: usize,
    length: usize,
}

impl Span {
    pub fn new((line, row): (usize, usize), length: usize) -> Self {
        Self { line, row, length }
    }
}

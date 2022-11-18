mod error;
pub mod parser;
#[allow(dead_code)]
pub mod symbols;
pub mod tokenizer;
pub use error::{Error, Result};

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Span {
    line: usize,
    column: usize,
    length: usize,
}

impl Span {
    pub fn new((line, row): (usize, usize), length: usize) -> Self {
        Self {
            line,
            column: row,
            length,
        }
    }
}

use std::fmt::Display;

use crate::tokenizer::TokenEnum;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[error("{row}:{column}: {error}")]
pub struct CompileError {
    row: usize,
    column: usize,
    error: CompileErrorEnum,
}

impl CompileError {
    pub fn new(row: usize, column: usize, error: CompileErrorEnum) -> Self {
        Self { row, column, error }
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum CompileErrorEnum {
    #[error("Unclosed string")]
    UnclosedString,

    #[error("Expected ''")]
    ExpectedPipe(String),

    #[error("Unexpected Token {0:?}")]
    UnexpectedToken(TokenEnum),

    #[error("Enexpected end of input")]
    UnexpectedEndOfInput { expected: PrintToken },

    #[error("Expected {expected}, found {found:?}")]
    ExpectedToken {
        expected: PrintToken,
        found: TokenEnum,
    },

    #[error("Expected one of {expected}, found {found:?}")]
    ExpectedOneOf {
        expected: PrintTokens,
        found: TokenEnum,
    },
}

macro_rules! token_name {
    ($token:expr) => {
        match $token {
            crate::tokenizer::TokenEnum::KWFunc => "func",
            crate::tokenizer::TokenEnum::KWData => "data",
            crate::tokenizer::TokenEnum::KWUsing => "using",
            crate::tokenizer::TokenEnum::KWObject => "object",
            crate::tokenizer::TokenEnum::KWEnum => "enum",
            crate::tokenizer::TokenEnum::KWRef => "ref",
            crate::tokenizer::TokenEnum::KWMut => "mut",
            crate::tokenizer::TokenEnum::KWPub => "pub",
            crate::tokenizer::TokenEnum::PipeMatch => "|?",
            crate::tokenizer::TokenEnum::PipeNext => "|>",
            crate::tokenizer::TokenEnum::PipeSet => "|=",
            crate::tokenizer::TokenEnum::PipeError => "|!",
            crate::tokenizer::TokenEnum::PipeReturn => "|.",
            crate::tokenizer::TokenEnum::PipeMatchEnd => "/?",
            crate::tokenizer::TokenEnum::LessThan => "<",
            crate::tokenizer::TokenEnum::GreaterThan => ">",
            crate::tokenizer::TokenEnum::Plus => "+",
            crate::tokenizer::TokenEnum::Minus => "-",
            crate::tokenizer::TokenEnum::Times => "*",
            crate::tokenizer::TokenEnum::Divide => "/",
            crate::tokenizer::TokenEnum::Power => "^",
            crate::tokenizer::TokenEnum::Modulo => "%",
            crate::tokenizer::TokenEnum::Arrow => "->",
            crate::tokenizer::TokenEnum::Or => "||",
            crate::tokenizer::TokenEnum::OpenBrace => "(",
            crate::tokenizer::TokenEnum::CloseBrace => ")",
            crate::tokenizer::TokenEnum::OpenCurlyBrace => "{",
            crate::tokenizer::TokenEnum::CloseCurlyBrace => "}",
            crate::tokenizer::TokenEnum::DoubleColon => "::",
            crate::tokenizer::TokenEnum::Colon => ":",
            crate::tokenizer::TokenEnum::SemiColon => ";",
            crate::tokenizer::TokenEnum::Period => ".",
            crate::tokenizer::TokenEnum::Comma => ",",
            crate::tokenizer::TokenEnum::Identifier(_) => "<identifier>",
            crate::tokenizer::TokenEnum::Bool(_) => "<true|false>",
            crate::tokenizer::TokenEnum::Integer(_) => "<integer>",
            crate::tokenizer::TokenEnum::Float(_) => "<float>",
            crate::tokenizer::TokenEnum::String(_) => "\"<string>\"",
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintTokens(&'static [TokenEnum]);

impl From<&'static [TokenEnum]> for PrintTokens {
    fn from(tokens: &'static [TokenEnum]) -> Self {
        Self(tokens)
    }
}

impl Display for PrintTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for token in self.0 {
            str += token_name!(token);
        }

        write!(f, "{str}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintToken(TokenEnum);

impl From<TokenEnum> for PrintToken {
    fn from(token: TokenEnum) -> Self {
        Self(token)
    }
}

impl Display for PrintToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", token_name!(self.0))
    }
}

#[cfg(test)]
mod tests;

use anyhow::anyhow;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum TokenEnum {
    /// func
    KWFunc,
    /// data
    KWData,
    /// using
    KWUsing,
    /// object
    KWObject,
    /// enum
    KWEnum,

    /// |?
    PipeMatch,
    /// |>
    PipeNext,
    /// |=
    PipeSet,
    /// |!
    PipeError,
    /// |.
    PipeReturn,
    /// \?
    PipeMatchEnd,

    /// <
    LessThen,
    /// >
    GreaterThen,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Times,
    /// /
    Divide,
    /// ^
    Power,
    /// %
    Modulo,
    /// ->
    Arrow,
    /// ||
    Or,

    /// (
    OpenBrace,
    /// )
    CloseBrace,
    /// {
    OpenCurlyBrace,
    /// }
    CloseCurlyBrace,
    /// ::
    DoubleColon,
    /// :
    Colon,
    /// ;
    SemiColon,
    /// .
    Period,

    Identifier(String),
    Bool(bool),
    Integer(i128),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub row: usize,
    pub column: usize,
    pub token: TokenEnum,
}

pub struct TokenIterItem {
    pub ch: char,
    pub row: usize,
    pub column: usize,
}

pub struct Tokenize<T: Iterator> {
    iter: Peekable<T>,
}

impl<T: Iterator<Item = TokenIterItem>> Iterator for Tokenize<T> {
    type Item = anyhow::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        get_next(&mut self.iter)
    }
}

pub trait TokenizeExt<T: Iterator<Item = TokenIterItem>> {
    fn tokenize(self) -> Tokenize<T>;
}

impl<T: Iterator<Item = TokenIterItem>> TokenizeExt<T> for T {
    fn tokenize(self) -> Tokenize<T> {
        Tokenize {
            iter: self.peekable(),
        }
    }
}

pub fn tokenize(source_code: &str) -> impl Iterator<Item = anyhow::Result<Token>> + '_ {
    Tokenize {
        iter: source_code
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(column, ch)| TokenIterItem {
                        ch,
                        column: column + 1,
                        row: row + 1,
                    })
            })
            .peekable(),
    }
}

fn get_next<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
) -> Option<anyhow::Result<Token>> {
    let TokenIterItem { ch, row, column } =
        iter.find(|TokenIterItem { ch, .. }| !ch.is_whitespace())?;

    Some(match ch {
        '|' => get_chain_operator(iter, column, row),
        ';' => Ok(Token {
            column,
            row,
            token: TokenEnum::SemiColon,
        }),
        '.' => Ok(Token {
            column,
            row,
            token: TokenEnum::Period,
        }),
        '(' => Ok(Token {
            column,
            row,
            token: TokenEnum::OpenBrace,
        }),
        ')' => Ok(Token {
            column,
            row,
            token: TokenEnum::CloseBrace,
        }),
        '{' => Ok(Token {
            column,
            row,
            token: TokenEnum::OpenCurlyBrace,
        }),
        '}' => Ok(Token {
            column,
            row,
            token: TokenEnum::CloseCurlyBrace,
        }),
        '+' => Ok(Token {
            column,
            row,
            token: TokenEnum::Plus,
        }),
        '-' => Ok(get_minus(iter, column, row)),
        '*' => Ok(Token {
            row,
            column,
            token: TokenEnum::Times,
        }),
        '/' => Ok(Token {
            row,
            column,
            token: TokenEnum::Divide,
        }),
        '%' => Ok(Token {
            row,
            column,
            token: TokenEnum::Modulo,
        }),
        '^' => Ok(Token {
            row,
            column,
            token: TokenEnum::Power,
        }),
        '<' => Ok(Token {
            row,
            column,
            token: TokenEnum::LessThen,
        }),
        '>' => Ok(Token {
            row,
            column,
            token: TokenEnum::GreaterThen,
        }),
        ':' => Ok(get_colon(iter, column, row)),
        '\\' => get_ends(iter, column, row),
        _ => Err(anyhow!("{row}:{column} Unexpected character {ch}")),
    })
}

fn get_chain_operator<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    column: usize,
    row: usize,
) -> anyhow::Result<Token> {
    let TokenIterItem {
        ch,
        row: row1,
        column: column1,
    } = iter
        .next()
        .ok_or_else(|| anyhow!("{row}:{column} Expected '>' '=' or '?' found EOF"))?;
    match ch {
        '>' => Ok(Token {
            column,
            row,
            token: TokenEnum::PipeNext,
        }),
        '=' => Ok(Token {
            column,
            row,
            token: TokenEnum::PipeSet,
        }),
        '?' => Ok(Token {
            column,
            row,
            token: TokenEnum::PipeMatch,
        }),
        '!' => Ok(Token {
            column,
            row,
            token: TokenEnum::PipeError,
        }),
        '.' => Ok(Token {
            column,
            row,
            token: TokenEnum::PipeReturn,
        }),
        '|' => Ok(Token {
            column,
            row,
            token: TokenEnum::Or,
        }),
        _ => Err(anyhow!("{row1}:{column1}: Unexpected character {ch}")),
    }
}

fn get_colon<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    column: usize,
    row: usize,
) -> Token {
    match iter.peek() {
        Some(TokenIterItem { ch: ':', .. }) => {
            iter.next();
            Token {
                row,
                column,
                token: TokenEnum::DoubleColon,
            }
        }
        _ => Token {
            column,
            row,
            token: TokenEnum::Colon,
        },
    }
}

fn get_minus<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    column: usize,
    row: usize,
) -> Token {
    match iter.peek() {
        Some(TokenIterItem { ch: '-', .. }) => {
            iter.next();
            Token {
                row,
                column,
                token: TokenEnum::Arrow,
            }
        }
        _ => Token {
            column,
            row,
            token: TokenEnum::Minus,
        },
    }
}

fn get_ends<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    column: usize,
    row: usize,
) -> anyhow::Result<Token> {
    let TokenIterItem {
        ch,
        row: row1,
        column: column1,
    } = iter
        .next()
        .ok_or_else(|| anyhow!("{row}:{column} Expected '?' found EOF"))?;
    match ch {
        '?' => Ok(Token {
            column,
            row,
            token: TokenEnum::PipeMatchEnd,
        }),
        _ => Err(anyhow!("{row1}:{column1}: Unexpected character {ch}")),
    }
}

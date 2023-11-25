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
    /// ref
    KWRef,
    /// mut
    KWMut,
    /// pub
    KWPub,

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
    /// ,
    Comma,

    Identifier(String),
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
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
    let mut row = 1;
    let mut column = 1;
    Tokenize {
        iter: source_code
            .chars()
            .map(move |char| match char {
                '\n' => {
                    let item = TokenIterItem {
                        ch: char,
                        row,
                        column: column + 1,
                    };

                    row += 1;
                    column = 1;

                    item
                }
                _ => {
                    let item = TokenIterItem {
                        ch: char,
                        row,
                        column,
                    };
                    column += 1;
                    item
                }
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
        '/' => return get_slash(iter, column, row),
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
        'a'..='z' | 'A'..='Z' | '_' => Ok(get_identifier(iter, ch, column, row)),
        '0'..='9' => Ok(get_number(iter, ch, column, row)),
        '"' => get_string(iter, column, row),
        ',' => Ok(Token {
            column,
            row,
            token: TokenEnum::Comma,
        }),
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
        Some(TokenIterItem { ch: '>', .. }) => {
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

fn get_identifier<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    current: char,
    column: usize,
    row: usize,
) -> Token {
    let mut str = format!("{current}");

    while {
        matches!(
            iter.peek(),
            Some(TokenIterItem {
                ch: 'a'..='z' | 'A'..='Z' | '_' | '0'..='9',
                ..
            })
        )
    } {
        str.push(iter.next().unwrap().ch);
    }

    Token {
        token: match str.as_str() {
            "data" => TokenEnum::KWData,
            "func" => TokenEnum::KWFunc,
            "using" => TokenEnum::KWUsing,
            "object" => TokenEnum::KWObject,
            "enum" => TokenEnum::KWEnum,
            "ref" => TokenEnum::KWRef,
            "mut" => TokenEnum::KWMut,
            "true" => TokenEnum::Bool(true),
            "false" => TokenEnum::Bool(false),

            _ => TokenEnum::Identifier(str),
        },
        column,
        row,
    }
}

fn get_number<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    current: char,
    column: usize,
    row: usize,
) -> Token {
    let mut str = format!("{current}");
    let mut found_decimal = false;

    while {
        match iter.peek() {
            Some(TokenIterItem { ch: '0'..='9', .. }) => true,
            Some(TokenIterItem { ch: '.', .. }) => {
                if !found_decimal {
                    found_decimal = true;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    } {
        str.push(iter.next().unwrap().ch);
    }

    Token {
        token: if str.contains('.') {
            TokenEnum::Float(str.parse().unwrap())
        } else {
            TokenEnum::Integer(str.parse().unwrap())
        },
        column,
        row,
    }
}

fn get_string<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    column: usize,
    row: usize,
) -> anyhow::Result<Token> {
    let mut str = String::new();
    let mut escape = false;

    while {
        match iter.peek() {
            Some(TokenIterItem { ch: '"', .. }) => {
                if escape {
                    escape = false;
                    str.push('"');
                    true
                } else {
                    false
                }
            }
            Some(TokenIterItem { ch: '\\', .. }) => {
                escape = true;
                true
            }
            Some(TokenIterItem { ch, .. }) => {
                str.push(*ch);
                true
            }

            None => {
                return Err(anyhow!(
                    "{row}:{column} Unclosed String, could not find closing '\"'"
                ))
            }
        }
    } {
        iter.next();
    }

    iter.next();

    Ok(Token {
        token: TokenEnum::String(str),
        column,
        row,
    })
}

fn get_slash<T: Iterator<Item = TokenIterItem>>(
    iter: &mut Peekable<T>,
    column: usize,
    row: usize,
) -> Option<anyhow::Result<Token>> {
    let next = iter.peek();

    match next {
        Some(TokenIterItem { ch: '/', .. }) => {
            while {
                match iter.peek() {
                    Some(TokenIterItem { ch: '\n', .. }) => false,
                    Some(_) => true,
                    None => false,
                }
            } {
                iter.next();
            }
            get_next(iter)
        }
        _ => Some(Ok(Token {
            token: TokenEnum::Divide,
            column,
            row,
        })),
    }
}

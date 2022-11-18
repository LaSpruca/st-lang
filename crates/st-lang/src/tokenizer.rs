use crate::{Error, Result, Span};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Token {
    pub token: TokenEnum,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenEnum {
    // Keyworkds
    FuncKW,
    StructKW,
    TraitKW,
    WithKW,
    ErrorKW,
    BeginKW,
    EndKW,
    LoopKW,
    LoopOverKW,
    RecoverKW,
    IfKW,
    ElseIfKW,
    ElseKW,
    CallKW,
    PtrKW,
    PackageKW,
    ModuleKW,
    UsingKW,
    LetKW,
    SetKW,
    NewKW,
    AsKW,
    PeekKW,
    SwapKW,
    DropKW,
    ReturnKW,
    PopKW,

    // Litterals
    Identifier(String),
    String(String),
    UInt(u64),
    Int(i64),
    Float(f64),
    Bool(bool),

    // Openy closey things &trade;
    ArrayOpen,
    ArrayClose,
    MapOpen,
    MapClose,
    TupleOpen,
    TupleClose,
    Colon,
    Comma,
}

impl TokenEnum {
    pub fn name(&self) -> String {
        match self {
            Self::Identifier(_) => "Identifier".into(),
            Self::String(_) => "String".into(),
            Self::UInt(_) => "UInt".into(),
            Self::Int(_) => "Int".into(),
            Self::Float(_) => "Float".into(),
            Self::Bool(_) => "Bool".into(),
            _ => {
                format!("{self:?}")
            }
        }
    }
}

pub fn tokenize(source: &str, file_name: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut characters: VecDeque<((usize, usize), char)> = source
        .split("\n")
        .enumerate()
        .map(|(line_no, line)| {
            format!("{line}\n")
                .chars()
                .enumerate()
                .map(move |(char_pos, character)| ((line_no, char_pos), character))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut prev_char = '-';
    let mut collector = "".to_string();
    let mut collector_start = (0, 0);

    macro_rules! push_collector {
        () => {
            if !collector.is_empty() {
                tokens.push(get_token(collector, collector_start, file_name)?);
                collector = String::new();
            }
        };
    }

    while let Some((start, character)) = characters.pop_front() {
        let start = (start.0 + 1, start.1 + 1);
        if collector.is_empty() {
            collector_start = start;
        }

        match (prev_char, character) {
            (_, '\'') => {
                push_collector!();
                let (string, reached_end) = get_until(&mut characters, "'", Some("\\"));

                if !reached_end {
                    return Err(Error::UnclosedString {
                        column: start.0,
                        line: start.1,
                        content: format!("\'{string}"),
                        file: file_name.to_owned(),
                    });
                }

                tokens.push(Token {
                    span: Span::new(start, string.len() + 2),
                    token: TokenEnum::String(string),
                })
            }
            (_, '\"') => {
                push_collector!();
                let (string, reached_end) = get_until(&mut characters, "\"", Some("\\"));

                if !reached_end {
                    return Err(Error::UnclosedString {
                        column: start.0,
                        line: start.1,
                        content: format!("\'{string}"),
                        file: file_name.to_owned(),
                    });
                }

                tokens.push(Token {
                    span: Span::new(start, string.len() + 2),
                    token: TokenEnum::String(string),
                })
            }
            ('-', '-') => {
                push_collector!();
                let _ = get_until(&mut characters, "\n", None);
            }
            (_, ' ' | '\n' | '\t') => {
                push_collector!();
            }
            (_, '[') => {
                push_collector!();
                tokens.push(Token {
                    token: TokenEnum::ArrayOpen,
                    span: Span::new(start, 1),
                })
            }
            (_, ']') => {
                push_collector!();
                tokens.push(Token {
                    token: TokenEnum::ArrayClose,
                    span: Span::new(start, 1),
                })
            }
            (_, '{') => {
                push_collector!();
                tokens.push(Token {
                    token: TokenEnum::MapOpen,
                    span: Span::new(start, 1),
                })
            }
            (_, '}') => {
                push_collector!();
                tokens.push(Token {
                    token: TokenEnum::MapClose,
                    span: Span::new(start, 1),
                })
            }
            (_, ',') => {
                push_collector!();
                tokens.push(Token {
                    token: TokenEnum::Comma,
                    span: Span::new(start, 1),
                })
            }
            (_, ':') => {
                push_collector!();
                tokens.push(Token {
                    token: TokenEnum::Colon,
                    span: Span::new(start, 1),
                })
            }

            _ => {
                collector += &character.to_string();
            }
        }
        prev_char = character;
    }

    Ok(tokens)
}

fn get_until(
    source: &mut VecDeque<((usize, usize), char)>,
    terminator: &str,
    escape_terminator: Option<&str>,
) -> (String, bool) {
    let mut captured = String::new();
    let escape_pattern =
        escape_terminator.map(|escape_terminator| format!("{escape_terminator}{terminator}"));

    if let Some(escape_pattern) = escape_pattern {
        while let Some((_, character)) = source.pop_front() {
            captured += &character.to_string();

            if captured.ends_with(terminator) && !captured.ends_with(&escape_pattern) {
                let len = captured.len();
                let len_start = len - terminator.len();
                captured.replace_range(len_start..len, "");
                return (captured, true);
            }
        }
    } else {
        while let Some((_, character)) = source.pop_front() {
            captured += &character.to_string();

            if captured.ends_with(terminator) {
                let len = captured.len();
                let len_start = len - terminator.len();
                captured.replace_range(len_start..len, "");
                return (captured, true);
            }
        }
    }

    (captured, false)
}

lazy_static! {
    static ref INT: Regex = Regex::new(r#"^-?\d[\d_]+$"#).unwrap();
    static ref UINT: Regex = Regex::new(r#"^\d[_\d]+u?$"#).unwrap();
    static ref FLOAT: Regex = Regex::new(r#"^\d[_\d]+(.\d+)?f?$"#).unwrap();
}

fn get_token(source: String, start: (usize, usize), file_name: &str) -> Result<Token> {
    Ok(Token {
        span: Span::new(start, source.len()),
        token: if INT.is_match(&source) {
            TokenEnum::Int(source.parse().map_err(|x| Error::IntParseError {
                file: file_name.into(),
                line: start.0,
                column: start.1,
                content: source.clone(),
                parse_error: x,
            })?)
        } else if UINT.is_match(&source) {
            TokenEnum::UInt(
                source
                    .replace("u", "")
                    .parse()
                    .map_err(|x| Error::IntParseError {
                        file: file_name.into(),
                        line: start.0,
                        column: start.1,
                        content: source.clone(),
                        parse_error: x,
                    })?,
            )
        } else if UINT.is_match(&source) {
            TokenEnum::UInt(
                source
                    .replace("u", "")
                    .parse()
                    .map_err(|x| Error::IntParseError {
                        file: file_name.into(),
                        line: start.0,
                        column: start.1,
                        content: source.clone(),
                        parse_error: x,
                    })?,
            )
        } else if FLOAT.is_match(&source) {
            TokenEnum::Float(source.replace("f", "").parse().map_err(|x| {
                Error::FloatParseError {
                    file: file_name.into(),
                    line: start.0,
                    column: start.1,
                    content: source.clone(),
                    parse_error: x,
                }
            })?)
        } else {
            match source.as_str() {
                "func" => TokenEnum::FuncKW,
                "struct" => TokenEnum::StructKW,
                "trait" => TokenEnum::TraitKW,
                "with" => TokenEnum::WithKW,
                "error" => TokenEnum::ErrorKW,
                "begin" => TokenEnum::BeginKW,
                "end" => TokenEnum::EndKW,
                "loop" => TokenEnum::LoopKW,
                "loop_over" => TokenEnum::LoopOverKW,
                "recover" => TokenEnum::RecoverKW,
                "if" => TokenEnum::IfKW,
                "else_if" => TokenEnum::ElseIfKW,
                "else" => TokenEnum::ElseKW,
                "call" => TokenEnum::CallKW,
                "ptr" => TokenEnum::PtrKW,
                "package" => TokenEnum::PackageKW,
                "module" => TokenEnum::ModuleKW,
                "using" => TokenEnum::UsingKW,
                "let" => TokenEnum::LetKW,
                "set" => TokenEnum::SetKW,
                "new" => TokenEnum::NewKW,
                "as" => TokenEnum::AsKW,
                "peek" => TokenEnum::PeekKW,
                "swap" => TokenEnum::SwapKW,
                "drop" => TokenEnum::DropKW,
                "return" => TokenEnum::ReturnKW,
                "pop" => TokenEnum::PopKW,
                _ => TokenEnum::Identifier(source),
            }
        },
    })
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref I64_REGEX: Regex = Regex::new("^[-]?[\\d_,]*$").unwrap();
    static ref U64_REGEX: Regex = Regex::new("^u[\\d_,]+$").unwrap();
    static ref F64_REGEX: Regex = Regex::new("^[-]?[\\d_,]*\\.[\\d_,]*$").unwrap();
}

enum State {
    Normal,
    MaybeComment,
    Comment,
    String,
    StringEscape,
}

#[derive(Debug, Clone)]
pub enum Token {
    StringLiteral(String),
    IntegerLiteral(i64),
    UnsignedIntegerLiteral(u64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Identifier(String),
    LetKW,
    WithKW,
    FuncKW,
    StructKW,
    BeginKW,
    EndKW,
}

pub fn parse(source: &String) -> Vec<Token> {
    let mut state = State::Normal;
    let mut error = false;

    let mut collector = String::new();
    let mut tokens: Vec<Token> = vec![];

    for char in source.chars() {
        match state {
            State::Normal => match char {
                ' ' | '\n' => {
                    if collector != "" {
                        tokens.push(parse_individual(collector));
                        collector = String::new();
                    }
                }
                '"' => {
                    state = State::String;
                }
                '-' => {
                    state = State::MaybeComment;
                }
                _ => {
                    collector += &char.to_string();
                }
            },
            State::Comment => {
                if char == '\n' {
                    state = State::Normal;
                }
            }
            State::MaybeComment => {
                if char == '-' {
                    state = State::Comment;
                }
            }
            State::String => {
                if char == '"' {
                    state = State::Normal;
                    tokens.push(Token::StringLiteral(collector));
                    collector = String::new();
                    continue;
                }

                if char == '\\' {
                    state = State::StringEscape;
                    continue;
                }

                collector += &char.to_string();
            }
            State::StringEscape => {
                match char {
                    '"' => {
                        collector += "\"";
                    }

                    'n' => {
                        collector += "\n";
                    }

                    '\\' => {
                        collector += "\\";
                    }

                    _ => {
                        error = true;
                        eprintln!("Could not find escape code \\{char}");
                    }
                };
                state = State::String;
            }
        };
    }

    if !collector.is_empty() {
        tokens.push(parse_individual(collector));
    }

    if error {
        panic!("Failed to parse source code");
    }

    return tokens;
}

fn parse_individual(text: String) -> Token {
    if U64_REGEX.is_match(&text) {
        return Token::UnsignedIntegerLiteral(
            text.strip_prefix("u")
                .expect("This should never fail")
                .replace(",", "")
                .parse()
                .expect("This should never fail"),
        );
    } else if I64_REGEX.is_match(&text) {
        return Token::IntegerLiteral(
            text.replace(",", "")
                .parse()
                .expect("This should never fail"),
        );
    } else if F64_REGEX.is_match(&text) {
        return Token::FloatLiteral(
            text.replace(",", "")
                .parse()
                .expect("This should never fail"),
        );
    } else if &text == "true" {
        return Token::BoolLiteral(true);
    } else if &text == "false" {
        return Token::BoolLiteral(false);
    } else if &text == "struct" {
        return Token::StructKW;
    } else if &text == "func" {
        return Token::FuncKW;
    } else if &text == "with" {
        return Token::WithKW;
    } else if &text == "begin" {
        return Token::BeginKW;
    } else if &text == "end" {
        return Token::EndKW;
    }

    return Token::Identifier(text);
}

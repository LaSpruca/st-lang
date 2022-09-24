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

#[derive(Debug)]
pub enum Command {
    PushStr(String),
    PushInt(i64),
    PushUInt(u64),
    PushFloat(f64),
    PushBool(bool),
    Call(String),
}

pub fn parse(source: &String) -> Vec<Command> {
    let mut state = State::Normal;
    let mut error = false;

    let mut collector = String::new();
    let mut commands: Vec<Command> = vec![];

    for char in source.chars() {
        match state {
            State::Normal => match char {
                ' ' | '\n' => {
                    if collector != "" {
                        commands.push(parse_individual(collector));
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
                    commands.push(Command::PushStr(collector));
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

    if error {
        panic!("Failed to parse source code");
    }

    return commands;
}

fn parse_individual(text: String) -> Command {
    if U64_REGEX.is_match(&text) {
        return Command::PushUInt(
            text.strip_prefix("u")
                .expect("This should never fail")
                .replace(",", "")
                .parse()
                .expect("This should never fail"),
        );
    } else if I64_REGEX.is_match(&text) {
        return Command::PushInt(
            text.replace(",", "")
                .parse()
                .expect("This should never fail"),
        );
    } else if F64_REGEX.is_match(&text) {
        return Command::PushFloat(
            text.replace(",", "")
                .parse()
                .expect("This should never fail"),
        );
    } else if &text == "true" {
        return Command::PushBool(true);
    } else if &text == "false" {
        return Command::PushBool(false);
    }

    return Command::Call(text);
}

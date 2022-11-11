use crate::Error;
use crate::Span;
use std::collections::VecDeque;

pub struct Token {
    pub token: TokenEnum,
    pub span: Span,
}

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
    UInt(String),
    Int(String),
    Float(String),
    Bool(String),

    // Openy closey thhings &trade;
    ArrayOpen,
    ArrayClose,
    MapOpen,
    MapClose,
    TupleOpen,
    TupleClose,
    Colon,
    Comma,
}

fn tokenize(source: &str, file_name: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut characters: VecDeque<((usize, usize), char)> = source
        .split("\n")
        .enumerate()
        .map(|(line_no, line)| {
            line.chars()
                .enumerate()
                .map(move |(char_pos, character)| ((line_no, char_pos), character))
        })
        .flatten()
        .collect();

    while let Some((start, character)) = characters.pop_front() {
        match character {
            '\'' => {}
            _ => {}
        }
    }

    Ok(tokens)
}

fn get_until(
    source: &mut VecDeque<((usize, usize), char)>,
    terminator: &str,
    escape_terminator: &str,
) -> (String, bool) {
    let mut captured = String::new();
    let escape_pattern = format!("{escape_terminator}{terminator}");

    while let Some((_, character)) = source.pop_front() {
        captured += &captured.to_string();

        if captured.ends_with(terminator) && !captured.ends_with(escape_pattern) {
            return (captured, true);
        }
    }

    (captured, false)
}

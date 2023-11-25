use super::*;

#[test]
fn test_pipe_operators() {
    let test_str = "|> |! |. |= |? \\?";
    let expected = vec![
        Ok(Token {
            token: TokenEnum::PipeNext,
            row: 1,
            column: 1,
        }),
        Ok(Token {
            token: TokenEnum::PipeError,
            row: 1,
            column: 4,
        }),
        Ok(Token {
            token: TokenEnum::PipeReturn,
            row: 1,
            column: 7,
        }),
        Ok(Token {
            token: TokenEnum::PipeSet,
            row: 1,
            column: 10,
        }),
        Ok(Token {
            token: TokenEnum::PipeMatch,
            row: 1,
            column: 13,
        }),
        Ok(Token {
            token: TokenEnum::PipeMatchEnd,
            row: 1,
            column: 16,
        }),
    ];

    let actual = tokenize(test_str)
        .map(|item| item.map_err(|e| e.to_string()))
        .collect::<Vec<_>>();

    assert_eq!(expected.len(), actual.len(), "{actual:#?}");

    expected
        .into_iter()
        .zip(actual)
        .enumerate()
        .for_each(|(i, (expected, received))| {
            assert_eq!(expected, received, "{}th assertion failed", i + 1)
        });
}

#[test]
fn parse_singles() {
    let test_str = "+-*/%^.(){}<>;:.";

    let expected = vec![
        Ok(Token {
            token: TokenEnum::Plus,
            row: 1,
            column: 1,
        }),
        Ok(Token {
            token: TokenEnum::Minus,
            row: 1,
            column: 2,
        }),
        Ok(Token {
            token: TokenEnum::Times,
            row: 1,
            column: 3,
        }),
        Ok(Token {
            token: TokenEnum::Divide,
            row: 1,
            column: 4,
        }),
        Ok(Token {
            token: TokenEnum::Modulo,
            row: 1,
            column: 5,
        }),
        Ok(Token {
            token: TokenEnum::Power,
            row: 1,
            column: 6,
        }),
        Ok(Token {
            token: TokenEnum::Period,
            row: 1,
            column: 7,
        }),
        Ok(Token {
            token: TokenEnum::OpenBrace,
            row: 1,
            column: 8,
        }),
        Ok(Token {
            token: TokenEnum::CloseBrace,
            row: 1,
            column: 9,
        }),
        Ok(Token {
            token: TokenEnum::OpenCurlyBrace,
            row: 1,
            column: 10,
        }),
        Ok(Token {
            token: TokenEnum::CloseCurlyBrace,
            row: 1,
            column: 11,
        }),
        Ok(Token {
            token: TokenEnum::LessThen,
            row: 1,
            column: 12,
        }),
        Ok(Token {
            token: TokenEnum::GreaterThen,
            row: 1,
            column: 13,
        }),
        Ok(Token {
            token: TokenEnum::SemiColon,
            row: 1,
            column: 14,
        }),
        Ok(Token {
            token: TokenEnum::Colon,
            row: 1,
            column: 15,
        }),
        Ok(Token {
            token: TokenEnum::Period,
            row: 1,
            column: 16,
        }),
    ];
    let actual = tokenize(test_str)
        .map(|item| item.map_err(|e| e.to_string()))
        .collect::<Vec<_>>();

    assert_eq!(expected.len(), actual.len(), "{actual:#?}");

    expected
        .into_iter()
        .zip(actual)
        .enumerate()
        .for_each(|(i, (expected, received))| {
            assert_eq!(expected, received, "{}th assertion failed", i + 1)
        });
}

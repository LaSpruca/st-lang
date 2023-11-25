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
    let test_str = "+-*/%^.(){}<>;:.,";

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
        Ok(Token {
            token: TokenEnum::Comma,
            row: 1,
            column: 17,
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
fn test_words() {
    let test_str = "func data using.object enum lalala true false mut ref";

    let expected = vec![
        Ok(Token {
            token: TokenEnum::KWFunc,
            row: 1,
            column: 1,
        }),
        Ok(Token {
            token: TokenEnum::KWData,
            row: 1,
            column: 6,
        }),
        Ok(Token {
            token: TokenEnum::KWUsing,
            row: 1,
            column: 11,
        }),
        Ok(Token {
            token: TokenEnum::Period,
            row: 1,
            column: 16,
        }),
        Ok(Token {
            token: TokenEnum::KWObject,
            row: 1,
            column: 17,
        }),
        Ok(Token {
            token: TokenEnum::KWEnum,
            row: 1,
            column: 24,
        }),
        Ok(Token {
            token: TokenEnum::Identifier("lalala".into()),
            row: 1,
            column: 29,
        }),
        Ok(Token {
            token: TokenEnum::Bool(true),
            row: 1,
            column: 36,
        }),
        Ok(Token {
            token: TokenEnum::Bool(false),
            row: 1,
            column: 41,
        }),
        Ok(Token {
            token: TokenEnum::KWMut,
            column: 47,
            row: 1,
        }),
        Ok(Token {
            token: TokenEnum::KWRef,
            column: 51,
            row: 1,
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
fn test_misc() {
    let test_str = "::->||";
    let expected = vec![
        Ok(Token {
            token: TokenEnum::DoubleColon,
            row: 1,
            column: 1,
        }),
        Ok(Token {
            token: TokenEnum::Arrow,
            row: 1,
            column: 3,
        }),
        Ok(Token {
            token: TokenEnum::Or,
            row: 1,
            column: 5,
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
fn test_parse_numbers() {
    let test_str = "123 123.123 123.123.123";
    let expected = vec![
        Ok(Token {
            token: TokenEnum::Integer(123),
            row: 1,
            column: 1,
        }),
        Ok(Token {
            token: TokenEnum::Float(123.123),
            row: 1,
            column: 5,
        }),
        Ok(Token {
            token: TokenEnum::Float(123.123),
            row: 1,
            column: 13,
        }),
        Ok(Token {
            token: TokenEnum::Period,
            row: 1,
            column: 20,
        }),
        Ok(Token {
            token: TokenEnum::Integer(123),
            row: 1,
            column: 21,
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
fn test_parse_str() {
    let test_str = r#""This is a \"beautiful\" 
string""#;
    let expected = vec![Ok(Token {
        token: TokenEnum::String("This is a \"beautiful\" \nstring".into()),
        row: 1,
        column: 1,
    })];

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
fn test_examples() {
    let files = glob::glob("../../examples/**/*.st").unwrap();
    let mut parsed = 0;

    for a in files {
        parsed += 1;
        let path = a.unwrap();
        let source = std::fs::read_to_string(&path).unwrap();
        let errors = tokenize(&source)
            .filter_map(|item| match item {
                Ok(_) => None,
                Err(e) => Some(e),
            })
            .collect::<Vec<_>>();
        assert_eq!(errors.len(), 0, "{}:\n{:#?}", path.display(), errors);
    }
    assert!(parsed > 0, "Parsed 0 files");
}

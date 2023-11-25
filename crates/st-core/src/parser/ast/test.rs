use super::*;

#[test]
fn test_parse_identifier() {
    let mut tokens = vec![
        Token {
            token: TokenEnum::Identifier("test".to_string()),
            row: 1,
            column: 1,
        },
        Token {
            token: TokenEnum::DoubleColon,
            row: 1,
            column: 2,
        },
        Token {
            token: TokenEnum::Identifier("test".to_string()),
            row: 1,
            column: 3,
        },
    ]
    .into_iter()
    .peekable();

    let result = parse_identifier(&mut tokens, "test".to_string()).map_err(|x| x.to_string());

    assert_eq!(
        result,
        Ok(AstNodeEnum::Identifier("test::test".to_string()))
    );
}

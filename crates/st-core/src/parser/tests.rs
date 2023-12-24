use super::*;

#[test]
fn test_match() {
    let parser = Parser::new()
        .r#match(
            |item| if item == 1 { Ok(()) } else { Err(Some(item)) },
            None,
        )
        .r#match(
            |item| if item == 2 { Ok(()) } else { Err(Some(item)) },
            None,
        )
        .r#match(
            |item| if item == 3 { Ok(()) } else { Err(Some(item)) },
            None,
        );

    assert_eq!(parser.execute(vec![1, 2, 3].into_iter()), Ok(()));
    assert_eq!(parser.execute(vec![1, 3, 3].into_iter()), Err(Some(3)));
    assert_eq!(parser.execute(vec![1].into_iter()), Err(None));
}

#[test]
fn test_take() {
    let parser = Parser::new()
        .r#match(
            |item| if item == 1 { Ok(()) } else { Err(Some(item)) },
            None,
        )
        .take(
            |item| {
                if item == 2 {
                    Ok("Two")
                } else {
                    Err(Some(item))
                }
            },
            None,
        )
        .take(
            |item| if item == 3 { Ok(3.0) } else { Err(Some(item)) },
            None,
        );

    assert_eq!(
        parser.execute(vec![1, 2, 3].into_iter()),
        Ok((((()), "Two"), 3.0))
    );
    assert_eq!(parser.execute(vec![1, 3, 3].into_iter()), Err(Some(3)));
    assert_eq!(parser.execute(vec![1].into_iter()), Err(None));
}

#[test]
fn test_transform() {
    let parser = Parser::new()
        .r#match(
            |item| if item == 1 { Ok(()) } else { Err(Some(item)) },
            None,
        )
        .take(
            |item| {
                if item == 2 {
                    Ok("Two")
                } else {
                    Err(Some(item))
                }
            },
            None,
        )
        .take(
            |item| if item == 3 { Ok(3.0) } else { Err(Some(item)) },
            None,
        )
        .transform(|((_, a), b)| (a, b));

    assert_eq!(parser.execute(vec![1, 2, 3].into_iter()), Ok(("Two", 3.0)));
    assert_eq!(parser.execute(vec![1, 3, 3].into_iter()), Err(Some(3)));
    assert_eq!(parser.execute(vec![1].into_iter()), Err(None));
}

#[test]
fn test_then() {
    let parser = Parser::new()
        .r#match(
            |item| if item == 1 { Ok(()) } else { Err(Some(item)) },
            None,
        )
        .then(
            Parser::new()
                .take(
                    |item| {
                        if item == 2 {
                            Ok("Two")
                        } else {
                            Err(Some(item))
                        }
                    },
                    None,
                )
                .take(
                    |item| if item == 3 { Ok(3.0) } else { Err(Some(item)) },
                    None,
                )
                .transform(|(a, b)| [a.to_string(), format!("{b}")]),
        );

    assert_eq!(
        parser.execute(vec![1, 2, 3].into_iter()),
        Ok(((), ["Two".into(), "3".into()]))
    );
    assert_eq!(parser.execute(vec![1, 3, 3].into_iter()), Err(Some(3)));
    assert_eq!(parser.execute(vec![1].into_iter()), Err(None));
}

use super::*;

#[macro_export]
macro_rules! parser {
    () => {::st_core::parser::Parser::new()};
    ({{ $e:expr }}) => {$e};
    ({ $e:expr }) => {$e};

    (take $case:pat => $item:expr, else $err:ident => $err_case:expr, $missing:expr; $($rest:tt)*) => {
        parser!(
            {{
                ::st_core::parser::parser_macro::take(
                    ::st_core::parser::Parser::new(),
                    |item| match item { $case => Ok($item), $err => Err($err_case) },
                    $missing
                )
            }}
            $($rest)*
        )
    };
    ({{ $parent:expr }} take $case:pat => $item:expr, else $err:ident => $err_case:expr, $missing:expr; $($rest:tt)*) => {
        parser!({{
                ::st_core::parser::parser_macro::take(
                    $parent,
                    |item| match item { $case => Ok($item), $err => Err($err_case) },
                    $missing
                )
            }}
            $($rest)*
        );
    };

    (match $case:pat, else $err:ident => $err_case:expr, $missing:expr; $($rest:tt)*) => {
        parser!(
            {{
                ::st_core::parser::parser_macro::r#match(
                    ::st_core::parser::Parser::new(),
                    |item| match item { $case => Ok(()), $err => Err($err_case) },
                    $missing
                )
            }}
            $($rest)*
        )
    };
    ({{ $parent:expr }} match $case:pat, else $err:ident => $err_case:expr, $missing:expr; $($rest:tt)*) => {
        parser!({{
                ::st_core::parser::parser_macro::r#match(
                    $parent,
                    |item| match item { $case => Ok(()), $err => Err($err_case) },
                    $missing
                )
            }}
            $($rest)*
        )
    };

    (then $parser_name:ident; $($rest:tt)*) => {
        parser!(
            {{
                ::st_core::parser::parser_macro::then(
                    ::st_core::parser::Parser::new(),
                    $parser_name
                )
            }}
            $($rest)*
        )
    };
    ({{ $parent:expr }} then $parser_name:ident; $($rest:tt)*) => {
        parser!({{
                ::st_core::parser::parser_macro::r#match($parent, $parser_name)
            }}
            $($rest)*
        )
    };

    (then { $($sub_parser:tt)* }; $($rest:tt)*) => {
        parser!(
            {{
                ::st_core::parser::parser_macro::then(
                    ::st_core::parser::Parser::new(),
                    parser!($($sub_parser)*)
                )
            }}
            $($rest)*
        )
    };
    ({{ $parent:expr }} then { $($sub_parser:tt)* }; $($rest:tt)*) => {
        parser!({{
                ::st_core::parser::parser_macro::r#then($parent, parser!($($sub_parser)*))
            }}
            $($rest)*
        )
    };
}

pub const fn r#match<Parent, F>(
    parent: Parent,
    matcher: F,
    missing: Parent::Error,
) -> Match<F, Parent>
where
    Parent: ParserBase,
    F: Fn(Parent::Item) -> Result<(), Parent::Error>,
{
    Match {
        matcher,
        missing,
        parent: parent,
    }
}

pub const fn take<Parent, I, F>(
    parent: Parent,
    matcher: F,
    missing: Parent::Error,
) -> Take<I, F, Parent>
where
    Parent: ParserBase,
    F: Fn(Parent::Item) -> Result<I, Parent::Error>,
{
    Take {
        matcher,
        missing,
        parent: parent,
    }
}

pub const fn transform<Parent, I, F>(parent: Parent, transformer: F) -> Transform<I, F, Parent>
where
    Parent: ParserBase,
    F: Fn(Parent::Output) -> I,
    Parent: ParserStep,
{
    Transform {
        transformer,
        parent: parent,
    }
}

pub const fn then<Parent, T>(parent: Parent, parser: T) -> Then<T, Parent>
where
    Parent: ParserBase,
    T: ParserStep<Item = Parent::Item, Error = Parent::Error>,
{
    Then {
        parser,
        parent: parent,
    }
}

pub const fn optional<Parent, I, F, P>(
    parent: Parent,
    test: F,
    parser: P,
) -> Optional<I, F, P, (), Parent>
where
    Parent: ParserBase,
    F: Fn(&Parent::Item) -> I,
    P: ParserStep<Item = Parent::Item, Error = Parent::Error>,
{
    Optional {
        test,
        parser,
        transform: (),
        parent: parent,
    }
}

pub const fn optional_take_transform<Parent, I, F, P, T, TO>(
    parent: Parent,
    test: F,
    parser: P,
    transform: T,
) -> Optional<Option<I>, F, P, T, Parent>
where
    Parent: ParserBase,
    F: Fn(&Parent::Item) -> Option<I>,
    P: ParserStep<Item = Parent::Item, Error = Parent::Error>,
    T: Fn(I, P::Output) -> TO,
{
    Optional {
        test,
        parser,
        transform,
        parent: parent,
    }
}

pub const fn optional_match_transform<Parent, F, P, T, TO>(
    parent: Parent,
    test: F,
    parser: P,
    transform: T,
) -> Optional<bool, F, P, T, Parent>
where
    Parent: ParserBase,
    F: Fn(&Parent::Item) -> bool,
    P: ParserStep<Item = Parent::Item, Error = Parent::Error>,
    T: Fn(P::Output) -> TO,
{
    Optional {
        test,
        parser,
        transform,
        parent: parent,
    }
}

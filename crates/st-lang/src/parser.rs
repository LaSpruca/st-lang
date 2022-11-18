use std::collections::{HashMap, VecDeque};

use crate::tokenizer::{Token, TokenEnum};
use crate::{Error, Span};

#[derive(Debug)]
pub struct ParsedSource {
    pub module: ModDef,
}

#[derive(Debug)]
pub enum ModDef {
    Package(Package),
}

#[derive(Debug)]
pub struct Package {
    name: String,
    version: String,
    dependencies: Vec<(String, String)>,
}

pub fn parse(source: Vec<Token>, file_name: &str) -> Result<ParsedSource, Vec<Error>> {
    let mut mod_def = None;
    let mut source = VecDeque::from(source);
    let mut errors = vec![];

    macro_rules! push_errors {
        ($a:expr) => {
            match $a {
                Err(ex) => {
                    errors.push(ex);
                    continue;
                }
                Ok(val) => val,
            }
        };
    }

    while let Some(Token { token, span }) = source.pop_front() {
        match token {
            TokenEnum::PackageKW => {
                mod_def = Some(ModDef::Package(push_errors!(parse_package(
                    &mut source,
                    file_name,
                    span
                ))));
            }
            _ => {}
        }
    }

    if let Some(module) = mod_def {
        ParsedSource { module }
    }
}

fn parse_package(
    source: &mut VecDeque<Token>,
    file_name: &str,
    start: Span,
) -> Result<Package, Vec<Error>> {
    let mut name = Option::<String>::None;
    let mut errors = Vec::new();
    let mut end_found = false;

    while let Some(Token { token, span }) = source.pop_front() {
        match token {
            TokenEnum::EndKW => {
                end_found = true;
                break;
            }
            TokenEnum::Identifier(ident) => match ident.as_str() {
                "name" => {
                    if let Some(Token { token, span }) = source.pop_front() {
                        match token {
                            TokenEnum::Identifier(pkg_name) => name = Some(pkg_name),
                            _ => errors.push(Error::UnexpectedToken {
                                file: file_name.to_owned(),
                                line: span.line,
                                column: span.column,
                                token: token.name(),
                                expected: "Identifier".to_owned(),
                            }),
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    if !end_found {
        errors.push(Error::UnclosedBlock {
            file: file_name.to_owned(),
            line: start.line,
            column: start.column,
            opening_block: "package".to_owned(),
        });
    } else {
        errors.push(Error::UnnamedModule {
            column: start.column,
            line: start.line,
            file: file_name.to_owned(),
        });
    }

    if errors.is_empty() {
        if let Some(name) = name {
            return Ok(Package {
                name: name,
                version: "0.0.0".to_string(),
                dependencies: vec![],
            });
        }
    } else {
        return Err(errors);
    }
}

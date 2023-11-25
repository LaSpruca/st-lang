#[cfg(test)]
mod test;

use std::iter::Peekable;

use crate::tokenizer::{Token, TokenEnum};

#[derive(PartialEq, Debug)]
pub struct AstNode {
    pub row: usize,
    pub column: usize,
    pub node: AstNodeEnum,
}

#[derive(PartialEq, Debug)]
pub enum AstNodeEnum {
    Identifier(String),
}

pub fn parse_identifier<Iter: Iterator<Item = Token>>(
    tokens: &mut Peekable<Iter>,
    mut identifier: String,
) -> anyhow::Result<AstNodeEnum> {
    loop {
        tokens.next();
        match tokens.peek() {
            Some(Token {
                token: TokenEnum::DoubleColon,
                ..
            }) => {
                identifier += "::";
            }
            _ => {
                break;
            }
        }
        tokens.next();

        match tokens.peek() {
            Some(Token {
                token: TokenEnum::Identifier(elem),
                ..
            }) => {
                identifier += elem;
            }
            Some(Token { row, column, token }) => {
                return Err(anyhow::anyhow!(
                    "{row}:{column}: Expected identifier, found {token:?}",
                ))
            }
            None => {
                return Err(anyhow::anyhow!("Expected identifier, found EOF"));
            }
        }
    }

    Ok(AstNodeEnum::Identifier(identifier))
}

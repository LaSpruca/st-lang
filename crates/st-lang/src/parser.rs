#[derive(thiserror::Error, Debug)]
pub enum ParserError {}

pub type Result<T> = core::result::Result<T, ParserError>;


pub enum Token {
    
}

fn parse(source: &str) -> Result<Vec<Token>> {

}
use core::panic;
use std::{env::current_exe, process::id};

use crate::{
    parser::Token,
    rt::module::{Function, FunctionSignature, Module},
};

macro_rules! push_literal {
    ($curr_fn:ident,$a:expr) => {
        $curr_fn.source_code.push(Command::PushLiteral($a))
    };
}

#[derive(Debug, Clone, Default)]
pub struct SourceFunction {
    source_code: Vec<Command>,
    signature: FunctionSignature,
    name: String,
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    UnsignedInteger(u64),
    Bool(bool),
    Float(f64),
}

#[derive(Clone, Debug)]
pub enum Command {
    PushLiteral(Literal),
    Call(String),
    PushIdentifier(String),
    Let { name: String, _type: String },
}

impl Function for SourceFunction {
    fn call(&self, context: &mut crate::rt::context::Context) {
        todo!("Implement the interpreter :)");
    }

    fn signature(&self) -> FunctionSignature {
        self.signature.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn build_module(source: &Vec<Token>) -> Module {
    let mut module = Module::default();

    let mut state_stack = vec![State::TopLevel];
    let mut current_function = SourceFunction::default();

    let mut error = false;

    for token in source {
        match state_stack.last().unwrap() {
            State::TopLevel => match token {
                Token::FuncKW => state_stack.push(State::FunctionDecl),
                Token::StructKW => state_stack.push(State::Struct),
                Token::StringLiteral(_) => {
                    eprintln!("Imagine having a working import system, couldn't be me");
                }
                _ => {
                    error = true;
                    eprintln!("Top level code unsupported, please only use string for imports ")
                }
            },
            State::FunctionDecl => match token {
                Token::WithKW => {
                    if current_function.name != "" {
                        state_stack.pop();
                        state_stack.push(State::FunctionDecl);
                    } else {
                        error = true;
                        eprintln!("Functions must have a name");
                    }
                }
                Token::FuncKW => {
                    error = true;
                    eprintln!("Unexpected Func, you may have meant the Function keyword");
                }
                Token::StructKW => {
                    error = true;
                    eprintln!("You can't just put a struct definition inside of a function and expect it to work");
                }
                Token::BeginKW => {
                    if current_function.name != "" {
                        state_stack.pop();
                        state_stack.push(State::FunctionBody);
                    } else {
                        error = true;
                        eprintln!("Functions must have a name");
                    }
                }
                Token::EndKW => {
                    error = true;
                    eprintln!("Please define a body for this god damn function");
                }
                Token::Identifier(ident) => {
                    if current_function.name == "" {
                        current_function.name = ident.to_string();
                    } else {
                        current_function.signature.stack_on.push(ident.to_string())
                    }
                }
                _ => {
                    error = true;
                    eprintln!("{token:?} can not appearer in function deceleration or params, tf you trina do?");
                }
            },
            State::FunctionParams => todo!(),
            State::FunctionBody => match token {
                Token::StringLiteral(string) => {
                    push_literal!(current_function, Literal::String(string.to_string()))
                }
                Token::IntegerLiteral(int) => {
                    push_literal!(current_function, Literal::Integer(*int))
                }
                Token::UnsignedIntegerLiteral(int) => {
                    push_literal!(current_function, Literal::UnsignedInteger(*int))
                }
                Token::FloatLiteral(float) => {
                    push_literal!(current_function, Literal::Float(*float));
                }
                Token::BoolLiteral(boolean) => {
                    push_literal!(current_function, Literal::Bool(*boolean))
                }
                Token::Identifier(identifier) => {
                    if identifier.starts_with("*") | identifier.starts_with("$") {
                        current_function
                            .source_code
                            .push(Command::PushIdentifier(identifier.to_string()))
                    } else {
                        current_function
                            .source_code
                            .push(Command::Call(identifier.to_string()))
                    }
                }
                Token::LetKW => {
                    state_stack.push(State::Let);
                }
                Token::WithKW => {
                    error = true;
                    eprintln!("With surves no purpose in function bodies as of yet");
                }
                Token::FuncKW => {
                    error = true;
                    eprintln!("Functions in function are not yet supported");
                }
                Token::StructKW => {
                    error = true;
                    eprintln!("You can't just put a struct definition inside of a function and expect it to work");
                }
                Token::BeginKW => {
                    error = true;
                    eprintln!("You have already begun this functions body, if you wanted a block, please wait, that's coming");
                }
                Token::EndKW => {
                    module.insert_function(current_function);
                    current_function = SourceFunction::default();
                }
            },
            State::Struct => todo!(),
            State::Let => todo!(),
        }
    }

    if error {
        panic!("There was an error");
    }

    module
}

enum State {
    TopLevel,
    FunctionDecl,
    FunctionParams,
    FunctionBody,
    Struct,
    Let,
}

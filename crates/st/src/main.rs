#![allow(dead_code)]

mod lexer;
mod parser;
mod rt;

use clap::arg;
use rt::context::Context;
use std::{fs, path::PathBuf};

fn main() {
    let cmd = clap::Command::new("st")
        .about("A stupid lil programmin")
        .arg(arg!(-'f' --"file" <PATH>).value_parser(clap::value_parser!(std::path::PathBuf)));

    let matches = cmd.get_matches();

    let path: &PathBuf = matches.get_one("file").expect("Please specify a file");

    let file = fs::read_to_string(path).expect("Could not find file");

    let source = parser::parse(&file);
    println!("{source:?}");
    let module = lexer::build_module(&source);
    println!("{module:?}");

    let mut context = Context::default();

    // rt::run(source, &mut context);
}

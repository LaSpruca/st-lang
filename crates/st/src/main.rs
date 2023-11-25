use clap::Parser;
use st_core::tokenizer::tokenize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    /// The source file to execute
    file: PathBuf,
    /// Use the source [`Args::file`] as a project manifest
    #[clap(long, short)]
    project: bool,
}

fn main() {
    let Args { file: path, .. } = Args::parse();

    let file = std::fs::read_to_string(&path).expect("Could not read file");

    let tokenizer = tokenize(&file);

    let _tokens = tokenizer.collect::<Vec<_>>();
}

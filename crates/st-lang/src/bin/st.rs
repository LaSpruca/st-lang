use st_lang::tokenizer::tokenize;

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let source = std::fs::read_to_string(&file).unwrap();
    let tokens = tokenize(&source, &file).unwrap();

    println!("{:#?}", tokens)
}

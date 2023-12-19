use argster::command;
use st_core::tokenizer::tokenize;
use std::path::PathBuf;

struct App;

#[command]
impl App {
    /// Run a source file
    /// # Args
    /// input The path to the source file
    /// --project -p Use the provided source file a project manifest
    fn run(input: PathBuf, project: bool) {
        _ = project;
        let path = input;
        let file = std::fs::read_to_string(&path).expect("Could not read file");
        let mut was_tokenize_error = false;

        let _tokenizer = tokenize(&file)
            .filter(|x| x.is_ok())
            .filter_map(|i| match i {
                Ok(val) => Some(val),
                Err(ex) => {
                    eprintln!("{}:{ex}", path.display());
                    was_tokenize_error = true;
                    None
                }
            });
    }
}

fn main() {
    App::main();
}

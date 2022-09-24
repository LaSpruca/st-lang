pub mod module;
mod parser;
pub mod runner;

#[macro_export]
macro_rules! add_fn {
    ($ctx:ident, $name:ident) => {
        $ctx.add_function(stringify! {$name}.to_string(), $name);
    };
}

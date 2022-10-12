use st::rt::module::Module;

#[no_mangle]
pub fn init() -> Module {
    Module::default()
}

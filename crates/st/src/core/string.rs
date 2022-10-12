use anyhow::anyhow;

use crate::rt::module::Type;

#[derive(Debug)]
pub struct VmString {
    data: String,
}

impl Type for VmString {
    fn call_method(
        &self,
        name: String,
        context: &mut crate::rt::context::Context,
    ) -> anyhow::Result<()> {
        match name {
            _ => Err(anyhow!("No method {name} on core/String")),
        }
    }
}

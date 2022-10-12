use std::{collections::HashMap, fmt::Debug};

use anyhow::Result;

use super::context::Context;

#[derive(Debug, Clone, Default)]
pub struct FunctionSignature {
    pub stack_on: Vec<String>,
    pub stack_off: Vec<String>,
}

pub trait Function: Debug {
    /// Call the actual function
    fn call(&self, context: &mut Context);

    /// The function signature, used for type checking
    fn signature(&self) -> FunctionSignature;

    /// The name of the function, should include module information, i.e. Core/numeric::+ or Core/dup
    fn name(&self) -> String;
}

pub trait Type: Debug {
    /// A function associated with the type, expects first top element to be a pointer to the given type.
    fn call_method(&self, name: String, context: &mut Context) -> Result<()>;
}

#[derive(Default, Debug)]
pub struct Module {
    /// List of all types in a give module
    types: HashMap<String, Box<dyn Type>>,
    /// List of all function in a give module
    functions: HashMap<String, Box<dyn Function>>,
    /// List of all submodules
    sub_modules: HashMap<String, Module>,
    /// The name of the module
    name: String,
}

impl Module {
    pub fn insert_function<T>(&mut self, function: T)
    where
        T: Function + 'static,
    {
        self.functions.insert(function.name(), Box::new(function));
    }

    pub fn add_function<T>(mut self, function: T) -> Self
    where
        T: Function + 'static,
    {
        self.functions.insert(function.name(), Box::new(function));
        self
    }
}

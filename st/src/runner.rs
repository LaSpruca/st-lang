use crate::parser::Command;
use core::panic;
use std::{collections::HashMap, fmt::Debug};

type Function = fn(ctx: &mut Context);

#[derive(Default)]
pub struct Type {
    name: String,
}

#[derive(Default)]
pub struct HeapItem {
    data: Vec<u8>,
    type_name: String,
}

impl Debug for HeapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeapItem")
            .field("data", &self.data)
            .field("methods", &self.methods.keys())
            .field("typ", &self.type_name)
            .finish()
    }
}

impl HeapItem {
    fn new(data: Vec<u8>, typ: String) -> Self {
        Self {
            data,
            type_name: typ,
            methods: HashMap::new(),
        }
    }

    fn call(&self, path: &str, context: &mut Context) -> Option<()> {
        self.methods.get(path)?(self, context);

        Some(())
    }

    fn add_method(&mut self, path: &str, method: Method) {
        self.methods.insert(path.to_string(), method);
    }

    fn string(value: String) -> Self {
        let mut slf = Self::new(value.as_bytes().to_owned(), "String".to_owned());

        slf.add_method("fmt", |slf, ctx| {
            let fmt_str = ctx.stack.pop().expect("Ran out of items to format with");
            if let StackItem::Ptr(ptr) = fmt_str {
                if let Some(value) = ctx.heap.get_mut(&ptr) {
                    if value.type_name != "String" {
                        panic!("Expected *String, found {}", fmt_str.name());
                    }

                    let text = String::from_utf8(value.data.clone())
                        .expect("How do you have an invalid string?");

                    let inner = String::from_utf8(slf.data.clone())
                        .expect("How do you have an invalid string?");

                    let replaced = text.replacen("{}", &inner, 1);

                    value.data = replaced.as_bytes().to_owned();
                } else {
                    panic!("SegFault, no context for you!");
                }
            } else {
                panic!("Expected *String, found {}", fmt_str.name())
            };
        });

        slf
    }
}

#[derive(Default)]

pub struct Context {
    pub stack: Vec<StackItem>,
    heap: HashMap<usize, HeapItem>,
    functions: HashMap<String, Function>,
    type_table: HashMap<String, Type>,
    count: usize,
}

impl Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("stack", &self.stack)
            .field("heap", &self.heap)
            .field("functions", &self.functions.keys())
            .field("count", &self.count)
            .finish()
    }
}

impl Context {
    fn push_heap(&mut self, value: HeapItem) {
        self.heap.insert(self.count, value);
        self.stack.push(StackItem::Ptr(self.count));

        self.count += 1;
    }

    pub fn add_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    pub fn call(&mut self, name: &str) -> Option<()> {
        let function = self.functions.get(name)?;
        function(self);
        Some(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StackItem {
    Ptr(usize),
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
}

impl StackItem {
    pub fn name(&self) -> String {
        match self {
            StackItem::Ptr(_) => "*unknown".to_string(),
            StackItem::Int(_) => "Int".to_string(),
            StackItem::UInt(_) => "UInt".to_string(),
            StackItem::Float(_) => "Float".to_string(),
            StackItem::Bool(_) => "Bool".to_string(),
        }
    }

    pub fn numeric(&self) -> Option<i64> {
        match self {
            StackItem::Int(x) => Some(*x),
            StackItem::UInt(x) => Some(*x as i64),
            StackItem::Float(x) => Some(x.round() as i64),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<f64> {
        match self {
            StackItem::Int(x) => Some(*x as f64),
            StackItem::UInt(x) => Some(*x as f64),
            StackItem::Float(x) => Some(*x),
            _ => None,
        }
    }
}

pub fn run(source: Vec<Command>, context: &mut Context) {
    for command in source.iter() {
        println!("{:?}", context);
        match command {
            Command::PushStr(str) => context.push_heap(HeapItem::string(str.to_owned())),
            Command::PushInt(val) => context.stack.push(StackItem::Int(*val)),
            Command::PushUInt(val) => context.stack.push(StackItem::UInt(*val)),
            Command::PushFloat(val) => context.stack.push(StackItem::Float(*val)),
            Command::PushBool(val) => context.stack.push(StackItem::Bool(*val)),
            Command::Call(_) => todo!(),
        }
    }
}

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum StackItem {
    Ptr { name: String, pointer: usize },
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
}

impl StackItem {
    pub fn name(&self) -> String {
        match self {
            StackItem::Ptr { name, .. } => format!("*{name}"),
            StackItem::Int(_) => "Int".to_string(),
            StackItem::UInt(_) => "UInt".to_string(),
            StackItem::Float(_) => "Float".to_string(),
            StackItem::Bool(_) => "Bool".to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct HeapItem {
    data: Vec<u8>,
    type_name: String,
}

impl HeapItem {
    pub fn new(data: Vec<u8>, typ: String) -> Self {
        Self {
            data,
            type_name: typ,
        }
    }

    pub fn type_name(&self) -> String {
        self.type_name.clone()
    }
}

#[derive(Default, Debug)]
pub struct Context {
    pub stack: Vec<StackItem>,
    pub heap: HashMap<usize, HeapItem>,
    count: usize,
}

impl Context {
    pub fn push_heap(&mut self, value: HeapItem) {
        let type_name = value.type_name();
        self.heap.insert(self.count, value);
        self.stack.push(StackItem::Ptr {
            name: type_name,
            pointer: self.count,
        });

        self.count += 1;
    }
}

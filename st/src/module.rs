use std::collections::HashMap;

use crate::runner::{Context, HeapItem};

type Method = fn(slf: &HeapItem, ctx: &mut Context);

pub struct Type {
    methods: HashMap<String, Method>,
}

pub struct Module {
    type_defs: HashMap<String, Type>,
}

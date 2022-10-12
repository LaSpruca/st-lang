use crate::rt::{
    context::{Context, StackItem},
    module::{Function, Module},
};

pub mod string;

pub fn core() -> Module {
    Module::default()
}

#[st_macros::st_function(params = [("a", "number"), ("b", "number")])]
fn plus(context: &mut Context) {}

// fn swap_top(context: &mut Context) {
//     if let Some(count) = context.stack.pop() {
//         if let StackItem::UInt(val) = count {
//             let val = val as usize;
//             let len = context.stack.len() - 1;

//             if val < context.stack.len() {
//                 context.stack.swap(len, len - val);
//             } else {
//                 panic!("{val} is greater then the stack");
//             }
//         } else {
//             panic!("Expected UInt, found {}", count.name())
//         }
//     } else {
//         panic!("Could not pop <count> from stack");
//     }
// }

// fn dup(ctx: &mut Context) {
//     let item = ctx
//         .stack
//         .get(ctx.stack.len())
//         .expect("Please have one item on the stack")
//         .clone();

//     ctx.stack.push(item);
// }

// fn f_plus(ctx: &mut Context) {
//     let a = ctx
//         .stack
//         .pop()
//         .expect("Expected two values on stack, found none")
//         .float()
//         .expect("Expected value to be numeric");

//     let b = ctx
//         .stack
//         .pop()
//         .expect("Expected two values on stack, found one")
//         .float()
//         .expect("Expected value to be numeric");

//     ctx.stack.push(StackItem::Float(a + b));
// }

// fn plus(ctx: &mut Context) {
//     let a = ctx.stack;

//     let b = ctx
//         .stack
//         .pop()
//         .expect("Expected two values on stack, found one")
//         .numeric()
//         .expect("Expected value to be numeric");

//     ctx.stack.push(StackItem::Int(a + b));
// }

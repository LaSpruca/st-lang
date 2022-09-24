use core::panic;

use st::{
    add_fn,
    runner::{Context, StackItem},
};

pub fn init(ctx: &mut Context) {
    add_fn!(ctx, swap_top);
}

fn swap_top(context: &mut Context) {
    if let Some(count) = context.stack.pop() {
        if let StackItem::UInt(val) = count {
            let val = val as usize;
            let len = context.stack.len() - 1;

            if val < context.stack.len() {
                context.stack.swap(len, len - val);
            } else {
                panic!("{val} is greater then the stack");
            }
        } else {
            panic!("Expected UInt, found {}", count.name())
        }
    } else {
        panic!("Could not pop <count> from stack");
    }
}

fn dup(ctx: &mut Context) {
    let item = ctx
        .stack
        .get(ctx.stack.len())
        .expect("Please have one item on the stack")
        .clone();

    ctx.stack.push(item);
}

fn f_plus(ctx: &mut Context) {
    let a = ctx
        .stack
        .pop()
        .expect("Expected two values on stack, found none")
        .float()
        .expect("Expected value to be numeric");

    let b = ctx
        .stack
        .pop()
        .expect("Expected two values on stack, found one")
        .float()
        .expect("Expected value to be numeric");

    ctx.stack.push(StackItem::Float(a + b));
}

fn p_plus(ctx: &mut Context) {
    let a = ctx
        .stack
        .pop()
        .expect("Expected two values on stack, found none")
        .numeric()
        .expect("Expected value to be numeric");

    let b = ctx
        .stack
        .pop()
        .expect("Expected two values on stack, found one")
        .numeric()
        .expect("Expected value to be numeric");

    ctx.stack.push(StackItem::Int(a + b));
}

fn print(ctx: &mut Context) {
    let format_str = 
}

#[cfg(test)]
mod tests {
    use crate::swap_top;
    use st::{add_fn, runner::Context};

    #[test]
    fn test_swap_top() {
        let mut ctx = Context::default();
        add_fn!(ctx, swap_top);

        ctx.stack.push(st::runner::StackItem::Bool(true));
        ctx.stack.push(st::runner::StackItem::Int(32));
        ctx.stack.push(st::runner::StackItem::UInt(12));

        ctx.stack.push(st::runner::StackItem::UInt(2));

        assert_eq!(ctx.call("swap_top"), Some(()));

        assert_eq!(
            ctx.stack,
            vec![
                st::runner::StackItem::UInt(12),
                st::runner::StackItem::Int(32),
                st::runner::StackItem::Bool(true),
            ]
        );
    }
}

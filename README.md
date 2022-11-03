# ST Lang

_A silly little stack based programming language_

---

# About

This is a silly little project I decided on embarking on. It is not intended to be particularly fast. The concept for this language was inspired by Tsodings series making [porth](https://gitlab.com/tsoding/porth). The language also has some inspiration from rust, and has it's interpreter written in rust. The main purpose of this language was just to see if I could do something like this.

## Basic Overview

ST Lang is a stack based, garbage collected, generic language, interpreted.

There are two main data locations in ST Lang, the stack and the Heap. The stack is a FILO data structure, where elements can be pushed on and poped off. This stack is unique for each block. It contains pointers to objects on the heap. The heap is where all of the data is stored. The heap contains all of the object references.

ST Lang supports generic types through the use of Traits, which are inspired by Rust.

# Basic syntax

## Comments

Currently only single line comments exist, denoted by two hyphens, `--.*\n`

## Keywords

| Keyword     | Uses                                                                                                                                 |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `func`      | function identifier, see [functions](#functions)                                                                                     |
| `struct`    | begin the definition of a struct, see [structures](#structures)                                                                      |
| `trait`     | begin a trait block, see [traits](#traits)                                                                                           |
| `with`      | usage varies on context, see [functions](#functions), [structures](#structs), [traits](#traits), or [loop_over](#loop_over) for uses |
| `error`     | used to specify errors, see [Error handling](#error-handling)                                                                        |
| `begin`     | open the body portion of a struct, trait, function, or loop                                                                          |
| `end`       | closes `begin`, `if`, `else_if`, `else`, `package`, `module` blocks                                                                  |
| `loop`      | loop identifier for looping while a condition is met see [Loop](#loop)                                                               |
| `loop_over` | loops over ever element in an iterator see [Loop over](#loop_over)                                                                   |
| `recover`   | used to recover after an error is thrown, see [Error handling](#error-handling)                                                      |
| `if`        | The start of an if block, see [Conditionals](#conditionals)                                                                          |
| `else_if`   | The start of an else if block, see [Conditionals](#conditionals)                                                                     |
| `else`      | The start of an else block, see [Conditionals](#conditionals)                                                                        |
| `call`      | call the function that has been pushed to the stack, see [anonyms functions](#anonymous-functions)                                   |
| `ptr`       | Puts the pointer of a named function onto the stack, see [Named functions](#named-functions)                                         |
| `package`   | denotes the start of a package deceleration see [Packages](#packages)                                                                |
| `module`    | denotes the start of a module deceleration, see [Modules](#modules)                                                                  |
| `using`     | Add a name to the namespace, see [Dependencies](#dependencies)                                                                       |
| `let`       | Start a let binding, see [locals](#locals)                                                                                           |
| `set`       | Used to set the value of a local binding, see [locals](#locals)                                                                      |
| `new`       | Used to instantiate a new instance of a structure, see [structures](#structures)                                                     |
| `as`        | Used to cast the type of a value form one trait type to another trait type, see [Typechecking](#typechecking)                        |

## Identifiers

| Identifier type | Description                                                    | See                       | syntax                      |
| --------------- | -------------------------------------------------------------- | ------------------------- | --------------------------- |
| path            | An identifier for something being exported by a package/module | [Modules](#modules)       | `(name\/)?(name)(::name)\*` |
| local           | A let binding in a function                                    | [Functions](#functions)   | `*(<name>)`                 |
| member          | A member of a struct when in a struct func region              | [Structures](#structures) | `$(<name>)`                 |

### Names

Names, often refereed to as `<name>` is the following pattern: `\p{L}[\p{L}\d]+`. This is the essential building block of identifiers in ST Lang.

## Literal

A Literal is a value that can be represented in source the translated to some sort of ST Lang type at runtime

| Name       | Description                                   | ST Lang Type             | Rust type                  | Syntax                            |
| ---------- | --------------------------------------------- | ------------------------ | -------------------------- | --------------------------------- |
| String     | A string of characters                        | `core/string::String`    | `std::string::String`      | `"[.\n\r]*"` or `'[.\n\r]*'`      |
| UInt       | An unsigned integer                           | `core/number::UInt`      | `u64`                      | `\d+u?`                           |
| Int        | A signed integer                              | `core/number::Int`       | `i64`                      | `-?\d+`                           |
| Float      | A 64bit floating point number                 | `core/number::Float`     | `f64`                      | `\d+(.\d+)?f?`                    |
| Bool       | True of false                                 | `core/bool::Bool `       | `bool`                     | `true` or `false`                 |
| Array{T}   | An array of elements of type T                | `core/array::Array`      | `std::vec::Vec<Ptr>`       | `[<item>(,<item>)*]`              |
| Map{T, U}  | A map with keys of the T and values of type U | `core/map::Map`          | `std::collection::HashMap` | `{<item>: value(,<item>:value)*}` |
| Tuple{...} | An ordered list of elements                   | `core/tuple::Tuple{...}` | `(...)`                    | `(<item>(,<item>)*)`              |

## Functions

Functions are blocks of code the can accept some parameter and return some value, the two types are named functions and anonymous functions. All functions map to the type `core/function::Function{T, V}`, where T is a tuple representing all of the parameters popped from the stack, and V is all the values pushed onto the stack at the end. Since function map to a ST lang type, they can be allocated on the heap and used as variables for parsing into other functions. When a function is invoked, the stack start empty, and it is expected that the listed return types should be left on the tail end of the stack. When wanting to push a pointer to a function to the stack, use the `ptr <name>` syntax to get a pointer to the current function. Not this pointer does not point to any variable on the stack, as functions are a static constant, the pointer just points to the function in the context of the module. This means that operations like `set` will fail when attempting to call them on a function pointer.

All functions are defined with the following syntax:

```st
func <name> <return types> with
    <parameters>
begin
    <function body>
end
```

The name of the function is a standard `name`. Next is the return types. This is a list of path identifiers, which list out how the stack should look at the end of the function from top to bottom. Next is the with statement. The `with` keyword separates the return types from the function parameters. The parameters are defined as `name Type`. The arguments to these parameters are provided as locals in the context of the function when invoked, and do not appear on the stack.

### Anonymous functions

Unlike a regular function, anonymous functions are defined on the heap, and are defined by making the name of a function \_, i.e.

```st
func _ <return types> with
    <parameters>
begin
    <function body>
end
```

Anonymous functions can only appear within other functions or methods.

## Structures

Structures repersent the structure of an object, and are defined with the following syntax

```st-lang
struct <name> with <traits> begin
    <fields>

    <methods>
end
```

or for structs that do not implement any traits

```
struct <name> begin
    <fields>

    <methods>
end
```

Once the type checker

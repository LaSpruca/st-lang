# ST Lang

_A silly little stack based programming language_

---

# About

This is a silly little project I decided on embarking on. It is not intended to be particularly fast. The concept for this language was inspired by Tsodings series making [porth](https://gitlab.com/tsoding/porth). The language also has some inspiration from rust, and has it's interpreter written in rust. The main purpose of this language was just to see if I could do something like this.

# Basic syntax

## Comments

Currently only single line comments exist, denoted by two hyphens, `--.*\n`

## Keywords

| Keyword     | Uses                                                                                                                                                                                                                                                                                                                                            |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `func`      | function identifier, see [functions](#functions)                                                                                                                                                                                                                                                                                                |
| `with`      | denote the separation between return values and function parameters, see [functions](#functions); denote the traits that a `struct` implements, see [structures](#structs); denote the dependencies of a `trait`, see [traits](#traits); denote the identifier of each element of an iterator in `loop_over` block, see [loop_over](#loop_over) |
| `begin`     | open the body portion of a struct, trait, or function                                                                                                                                                                                                                                                                                           |
| `end`       | close a begin block; close an else block; close a package block; close a module block                                                                                                                                                                                                                                                           |
| `loop`      | loop identifier for looping while a condition is met see [loop](#loop)                                                                                                                                                                                                                                                                          |
| `loop_over` | loops over ever element in an iterator see [loop_over](#loop_over)                                                                                                                                                                                                                                                                              |
| `if`        | The start of an if block                                                                                                                                                                                                                                                                                                                        |
| `else_if`   | The start of an else if block                                                                                                                                                                                                                                                                                                                   |
| `else`      | The start of an else block                                                                                                                                                                                                                                                                                                                      |
| `struct`    | begin the definition of a struct, see [structures](#structures)                                                                                                                                                                                                                                                                                 |
| `trait`     | begin a trait block, see [traits](#traits)                                                                                                                                                                                                                                                                                                      |
| `call`      | call the function that has been pushed to the stack, see [anonyms functions](#anonymous-functions)                                                                                                                                                                                                                                              |
| `package`   | denotes the start of a package deceleration see [Packages](#packages)                                                                                                                                                                                                                                                                           |
| `module`    | denotes the start of a module deceleration, see [Modules](#modules)                                                                                                                                                                                                                                                                             |
| `require`   | The start of a dependencies, see [Dependencies](#dependencies)                                                                                                                                                                                                                                                                                  |
| `require`   | The start of a dependencies, see [Dependencies](#dependencies)                                                                                                                                                                                                                                                                                  |
| `using`     | Add a name to the namespace, see [Dependencies](#dependencies)                                                                                                                                                                                                                                                                                  |
| `include`   | Add a file to the current package, see [packages](#packages)                                                                                                                                                                                                                                                                                    |
| `let`       | Start a let binding, see [locals](#locals)                                                                                                                                                                                                                                                                                                      |

## Identifiers

| Identifier type | Description                                                    | See                       | syntax                      |
| --------------- | -------------------------------------------------------------- | ------------------------- | --------------------------- |
| path            | An identifier for something being exported by a package/module | [Modules](#modules)       | `(part\/)?(part)(::part)\*` |
| local           | A let binding in a function                                    | [Functions](#functions)   | `*(<part>)`                 |
| member          | A member of a struct when in a struct func region              | [Structures](#structures) | `$(<part>)`                 |

## Pushing to the stack

Values are pushed to the stack by either typing out a value identifier or a literal value.

### Literal

A Literal is a value that can be represented in source the translated to some sort of ST Lang type at runtime

| Name      | Description                                   | ST Lang Type          | Rust type                  | Syntax                           |
| --------- | --------------------------------------------- | --------------------- | -------------------------- | -------------------------------- |
| String    | A string of characters                        | `core/string::String` | `std::string::String`      | `"[.\n\r]*"` or `'[.\n\r]*'`     |
| UInt      | An unsigned integer                           | `core/number::UInt`   | `u64`                      | `\d+u?`                          |
| Int       | A signed integer                              | `core/number::Int`    | `i64`                      | `-?\d+`                          |
| Float     | A 64bit floating point number                 | `core/number::Float`  | `f64`                      | `\d+(.\d+)?f?`                   |
| Bool      | True of false                                 | `core/bool::Bool `    | `bool`                     | `true` or `false`                |
| Array{T}  | An array of elements of type T                | `core/array::Array`   | `std::vec::Vec<Ptr>`       | `[<item>(,<item>)*]`             |
| Map{T, U} | A map with keys of the T and values of type U | `core/map::Map`       | `std::collection::HashMap` | `{<item>: value(,<item>:value)}` |

## Functions

Functions are blocks of code the can accept some parameter and return some value, This

### Anonymous functions

Any series of characters defined in a string are able to store

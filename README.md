# ST Lang

_A silly little stack based programming language_

---

# About

This is a silly little project I decided on embarking on. It is not intended to be particularly fast. The concept for this language was inspired by Tsodings series making [porth](https://gitlab.com/tsoding/porth). The language also has some inspiration from rust and typescript, and has it's interpreter written in rust. The main purpose of this language was just to see if I could do something like this.

## Basic Overview

ST Lang is a stack based, garbage collected, generic language, interpreted.

There are two main data locations in ST Lang, the stack and the Heap. The stack is a FILO data structure, where elements can be pushed on and poped off. This stack is unique for each block. It contains pointers to objects on the heap. The heap is where all of the data is stored. The heap contains all of the object references.

ST Lang supports generic types through the use of Traits, which are inspired by Rust.

# Comments

Currently only single line comments exist, denoted by two hyphens, `--.*\n`

# Keywords

| Keyword     | Uses                                                                                                                                                 |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `func`      | function identifier, see [functions](#functions)                                                                                                     |
| `struct`    | begin the definition of a struct, see [structures](#structures)                                                                                      |
| `trait`     | begin a trait block, see [traits](#traits)                                                                                                           |
| `with`      | usage varies on context, see [functions](#functions), [structures](#structs), [traits](#traits), or [loop_over](#loop_over) for uses                 |
| `error`     | used to specify errors, see [Error handling](#error-handling)                                                                                        |
| `begin`     | open the body portion of a struct, trait, function, or loop                                                                                          |
| `end`       | closes `begin`, `if`, `else_if`, `else`, `package`, `module` blocks                                                                                  |
| `loop`      | loop identifier for looping while a condition is met see [Loop](#loop)                                                                               |
| `loop_over` | loops over ever element in an iterator see [Loop over](#loop_over)                                                                                   |
| `recover`   | used to recover after an error is thrown, see [Error handling](#error-handling)                                                                      |
| `if`        | The start of an if block, see [Conditionals](#conditionals)                                                                                          |
| `else_if`   | The start of an else if block, see [Conditionals](#conditionals)                                                                                     |
| `else`      | The start of an else block, see [Conditionals](#conditionals)                                                                                        |
| `call`      | call the function that has been pushed to the stack, see [anonyms functions](#anonymous-functions)                                                   |
| `ptr`       | Puts the pointer of a named function onto the stack, see [Named functions](#named-functions)                                                         |
| `package`   | denotes the start of a package deceleration see [Packages](#packages)                                                                                |
| `module`    | denotes the start of a module deceleration, see [Modules](#modules)                                                                                  |
| `using`     | Add a name to the namespace, see [Dependencies](#dependencies)                                                                                       |
| `let`       | Start a let binding, see [locals](#locals)                                                                                                           |
| `set`       | Used to set the value of a local binding, see [locals](#locals)                                                                                      |
| `new`       | Used to instantiate a new instance of a structure, see [structures](#structures)                                                                     |
| `as`        | Used to cast the type of a value form one trait type to another trait type, see [Typechecking](#typechecking)                                        |
| `peek`      | Take the top value of the stack inplace, see [Stack operations](#stack-manipulation)                                                                 |
| `swap`      | Swapping two values on the stack, see [Stack operations](#stack-manipulation)                                                                        |
| `drop`      | Remove the top item from the stack, see [Stack operations](#stack-manipulation)                                                                      |
| `return`    | Returns early from a function, see [Functions](#functions)                                                                                           |
| `pop`       | Removes the top value from the stack and enable you to use it in sutiuations like constructing an array, see [Stack operations](#stack-manipulation) |

# Identifiers

| Identifier type | Description                                                    | See                       | syntax                      |
| --------------- | -------------------------------------------------------------- | ------------------------- | --------------------------- |
| path            | An identifier for something being exported by a package/module | [Modules](#modules)       | `(name\/)?(name)(::name)\*` |
| local           | A let binding in a function                                    | [Functions](#functions)   | `*(<name>)`                 |
| member          | A member of a struct when in a struct func region              | [Structures](#structures) | `$(<name>)`                 |

## Names

Names, often refereed to as `<name>` is the following pattern: `\p{L}[\p{L}\d]+`. This is the essential building block of identifiers in ST Lang.

# Literal

A Literal is a value that can be represented in source the translated to some sort of ST Lang type at runtime

| Name       | Description                                   | ST Lang Type               | Rust type                  | Syntax                            |
| ---------- | --------------------------------------------- | -------------------------- | -------------------------- | --------------------------------- |
| String     | A string of characters                        | `core/string::String`      | `std::string::String`      | `"[.\n\r]*"` or `'[.\n\r]*'`      |
| UInt       | An unsigned integer                           | `core/number::UInt`        | `u64`                      | `\d+u?`                           |
| Int        | A signed integer                              | `core/number::Int`         | `i64`                      | `-?\d+`                           |
| Float      | A 64bit floating point number                 | `core/number::Float`       | `f64`                      | `\d+(.\d+)?f?`                    |
| Bool       | True of false                                 | `core/bool::Bool `         | `bool`                     | `true` or `false`                 |
| Array{T}   | An array of elements of type T                | `core/array::Array`        | `std::vec::Vec<Ptr>`       | `[<item>(,<item>)*]`              |
| Map{T, U}  | A map with keys of the T and values of type U | `core/map::Map`            | `std::collection::HashMap` | `{<item>: value(,<item>:value)*}` |
| Tuple{...} | An ordered list of elements                   | `core/tuple::Tuple{...}`   | `(...)`                    | `(<item>(,<item>)*)`              |
| None       | Create an Option instance with no value       | `core/optional::Option{T}` | `std::option::Option<T>`   | `None`                            |
| Some       | Create an Option with a value                 | `core/optional::Option{T}` | `std::option::Option<T>`   | `<item> Some`                     |

# Functions

Functions are blocks of code the can accept some parameter and return some value, the two types are named functions and anonymous functions. All functions map to the type `core/function::Function{T, V}`, where T is a tuple representing all of the parameters popped from the stack, and V is all the values pushed onto the stack at the end. Since function map to a ST lang type, they can be allocated on the heap and used as variables for parsing into other functions. When a function is invoked, the stack start empty, and it is expected that the listed return types should be left on the tail end of the stack. When wanting to push a pointer to a function to the stack, use the `ptr <name>` syntax to get a pointer to the current function. Not this pointer does not point to any variable on the stack, as functions are a static constant, the pointer just points to the function in the context of the module. This means that operations like `set` will fail when attempting to call them on a function pointer. By default, when the end of a function is reached, the stack is returned and appended onto the top of the callers stack. However, if you want to return early, you can use the `return` keyword, to return the current stack to the top of the callers stack, provided the typechecking says it's OK.

All functions are defined with the following syntax:

```st
func <name> <return types> with
    <parameters>
begin
    <function body>
end
```

The name of the function is a standard `name`. Next is the return types. This is a list of path identifiers, which list out how the stack should look at the end of the function from top to bottom. Next is the with statement. The `with` keyword separates the return types from the function parameters. The parameters are defined as `name Type`. The arguments to these parameters are provided as locals in the context of the function when invoked, and do not appear on the stack.

Functions that have a chance of erroring need to denote what the do if they error using the error identifier.

```st
func <name> <return types> error <error return types> with
    <parameters>
begin
    <function body>
end
```

The error return types is the return type of the function in the case that it errors.

## Anonymous functions

Unlike a regular function, anonymous functions are defined on the heap, and are defined by making the name of a function \_, i.e.

```st
func _ <return types> with
    <parameters>
begin
    <function body>
end
```

Anonymous functions can only appear within other functions or methods.

# Structures

Structures repersent the structure of an object, and are defined with the following syntax

```st
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

Every field in a struct has an associated getter method generated so that it's contents can be accessed from outside a method, i.e. in a function or a method on another struct etc., however, you will need to manually create a setter method for any fields you want to be able to set.

Fields or methods that you don't want to expose to the outside world, i.e. to anything outside of the current struct, can be annotated with the `internal` keyword like so

```st
internal name String
```

This means that methods cannot be accessed outside the currect struct. Any internal fields will will not gain a generated getter method, and instead, the only mean of accessing these field is to use the field accessor inside of a method.

## Methods

Methods are defined in the same way as a normal structure. These only difference is that the method allows for accessing of a structs directly fields with a member identifier. A structs methods call also be referenced using a member identifier. This will remove any need to pass the reference to the current object into the function call. Here is a small example of what that looks like

```st
struct Foo
    a String

    func bar1 with begin
        "Help" $a set
        $print
    end

    func print with begin
        $a putstrln
    end
end
```

To call a method outside of struct itself, you call the method like any normal function, making sure the object instance is on the top of the stack.

## Setting fields

Fields can be set using the `set` keyword like a so

```st
<value> $<field name> set
```

## Instantiating new instances

When you want to create a new instance of a struct, there are two main parts that are required, the new method, and then the new keyword. The new method _must_ set all of the fields on a struct, or the type checker will fail. If a fields does not need to have a value immediately, and can't have a default value, then it should be an optional type with the value of none. You can the use `new <struct>` to create the instance as if it were a normal function

# Traits

Traits in ST Lang provide a set of methods that a certain struct should implement. This is done by first annotating that a struct implements a trait as shown in the [structures](#structures) section, and then implementing methods with the name `<trait name>::<trait method>`.

The definition for a trait is a follows

```st
trait <name> begin
    <trait methods>
end
```

or if your trait also depends on some other traits, you can use a with clause

```st
trait <name> with <traits> begin
    <trait methods>
end
```

## Trait methods

Trait methods can either be abstract or provide some sort of default implementation. Anonyms traits methods have the following signature:

```st
trait Foo begin
    func <name> <return types> with <parameters> end
end
```

while trait methods with default implementations are defined just like normal methods. Trait methods can access other trait methods defined in the current trait like in a struct using member identifiers.

When invoking a trait method on a concrete type, you can use the normal `<struct>::<method>` syntax, however, if you have a trait type, you should use `<trait>::<method>` instead.

# Locals

Locals are a store of pointers within a function that can be referenced by name. The can be created with the following syntax

```st
let <name> <Type>
```

The pointer can then be assigned with the `set` keyword using the following syntax

```st
<value> *<local> set
```

Before any function can be called using a local, they must have a set value. If you want to set an empty value, make the type of your local `Option{T}` and set the value to none using `None *<local> set`.

# Optionals

If you need variables to have a value that is not set as soon as it is initialized, then you can use the `core/optional::Option{T}` type. This type is a replacement for something like null in other language, and is a wrapper used to indicate when a value is missing. The two way to construct a Option are

```st
None
```

and

```st
<value> Some
```

as mentioned in the [literals table](#literal).

The value of a optional can the be access via the Option::value method. This method will error if the value of the underling option is None.

# Error handling

Error handling is done using the error and recover keywords

## Indicating errors

A function that can error is denoted with the `error` keyword. The error keyword is the followed by the return type of the function if it does error as mentioned in the [function section](#functions).

When you want to return the stack as an error, you can use the `error` keyword again and it will take the contents of the stack and return as an error

## Recovery

To recover from an error, you can use the recover block. This block provides the error to correct the types on the stack to match if the function has succeeded or return/error from the current function, panic, or exit gracefully. It is used in the following way

```st
function_that_errors

recover
    <error handler>
end
```

If no recover block is specified, then the program will just panic out of existence. 'T was nice knowing you!

# Stack manipulation

## Swap

When you want to swap the positions of to values in the stack, you can use the swap keyword:

```
<element 2> <element 1> swap
```

Element 1 and element 2 are the indexes of the elements on the stack

## Peek/Pop

Peak is a placeholder that takes the value of the current element on the stack, allowing you to use stack elements in array or map construtction for example. It can be done like a so

```st
5
[peak]
```

Pop does the same thing as peek, however pop removes the element from the stack while pop does not.

## Delete

Delete removes the top element from the stack

# Typechecking

Typechecking is done though the use of an intermediate typechecking stack. At the end of any block, the stack should be left empty, except in the case of a function, where whatever is left on the stack should match the return type

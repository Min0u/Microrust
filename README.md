# µRust #

µRust is a small, simple, and fast interpreter for the Rust programming language.

## Features ##

- **Arithmetic Evaluation**: Supports basic arithmetic operations including addition, subtraction, multiplication, and division.
- **Variable Declaration and Assignment**: Allows the declaration and assignment of variables using the let keyword. Variables can hold integer values.
- **Variable Scoping**: Follows lexical scoping rules. Variables declared in inner scopes can shadow outer variables with the same name.
- **Error Handling**: Provides informative error messages for various scenarios including division by zero, undefined identifiers, redefinition of variables, and type mismatches.
- **Control Flow**: Use if-else statements, while loops and ternary expressions (`(condition) ? true_expr : false_expr`)
- **Pointer Representation**: Displays the memory address and value of variables using pointer notation (&variable).

## Installation ##
To install µRust, clone the repository and run the following command:

```bash
cargo install --path .
```

## Usage ##
To run the µRust interpreter, use the following command:

```bash
cargo run
```

## Supported Operations ##
- **Arithmetic Operations**: +, -, *, /
- **Conditional Operations**: ==, !=, <, >, <=, >=
- **Logical Operations**: &&, ||
- **Assignment Operator**: let
- **Conditional Operator**: ? :

## Examples ##

```rust
// Addition
µRust # 1 + 1
- : isize = 2

// Subtraction
µRust # 1 + 2 - 3 * 4 / 5
- : isize = 1

// Division + Error Handling
µRust # 1 / (1 - 1)
Evaluation Error: Division by zero, `(1 - 1)` evaluates to 0

// Declaration and assignment
µRust # let one = 1
one : isize = 1
µRust # one
- : isize = 1
µRust # one + 1
- : isize = 2

// Reassignment
µRust # let repeat = 0
repeat : isize = 0
µRust # let repeat = 0
Evaluation Error: Identifier `repeat` already defined.

// Ternary expressions
µRust # (one == 1) ? 1 : 2
- : isize = 1
ÂµRust # (one != 1) ? 1 : 2
- : isize = 2

// if-else statements
µRust # let mut acc = 0
acc : isize = 0
µRust # acc = acc + 1
- : isize = 1
µRust # if (acc == 0) { acc = acc + 1} else { acc = acc - 1}
- : isize = 0

// while loops
µRust # let mut k = 0
k : isize = 0
µRust # while (k < 4) {k = k + 3}
- : unit = ()
µRust # k
- : isize = 6

// Pointer representation
µRust # let x = 0
x : isize = 0
µRust # &x
- : Ptr = @[0, x]
µRust # {let x = 8; &x}
- : Ptr = @[1, x]
µRust # {let y = 8; &x}
- : Ptr = @[0, x]
```

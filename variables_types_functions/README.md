# Rust Fundamentals: Variables, Types, and Control Flow

This project demonstrates fundamental Rust programming concepts including variables, data types, and control flow structures.

## Table of Contents
- [Variables and Mutability](#variables-and-mutability)
- [Data Types](#data-types)
  - [Scalar Types](#scalar-types)
  - [Compound Types](#compound-types)
- [Control Flow](#control-flow)
  - [Conditionals](#conditionals)
  - [Loops](#loops)
  - [Loop Labels](#loop-labels)
- [Functions](#functions)
- [Integer Types](#integer-types)

## Variables and Mutability

### Key Concepts:
- Variables are immutable by default in Rust
- Use `let` to declare variables
- Use `mut` to make variables mutable
- **Shadowing vs `mut`**: 
  - `mut` allows modifying the value of a variable
  - Shadowing creates a new variable with the same name, allowing you to change its type or value
  - Use shadowing when you want to transform a value to a different type or when you want to reuse a variable name
  - Use `mut` when you need to modify a value in place
- Use `mut` to make variables mutable
- Shadowing allows reusing variable names
- Constants use `const` and require type annotations

### Example:
```rust
let x = 5;          // Immutable variable
let mut y = 5;      // Mutable variable
const Z: i32 = 100; // Constant

// Shadowing
let x = x + 1;      // New variable that shadows the previous x
```

## Data Types

### Scalar Types
- **Integers**: 
  - **Signed** (i8, i16, i32, i64, i128, isize): Can represent both positive and negative numbers
  - **Unsigned** (u8, u16, u32, u64, u128, usize): Can only represent zero and positive numbers
  - Signed integers use two's complement representation
  - Unsigned integers can represent larger positive numbers than their signed counterparts of the same size
  - Choose based on whether you need negative numbers and the expected range of values
- **Floating-point**: f32 and f64
- **Boolean**: true or false
- **Character**: Single Unicode scalar value

### Compound Types
- **Tuples**: Fixed-length collection of values with different types
- **Arrays**: Fixed-length collection of values with the same type

### Example:
```rust
// Tuple
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // Destructuring

// Array
let a = [1, 2, 3, 4, 5];
let first = a[0]; // Accessing elements
```

## Control Flow

### Conditionals
- `if` expressions for conditional execution
- `else if` for multiple conditions
- Using `if` in `let` statements

### Loops
- `loop`: Infinite loop until explicitly broken
- `while`: Loop while condition is true
- `for`: Iterate over collections or ranges

### Loop Labels
- Use labels with `'label_name:` to specify which loop to break/continue

### Example:
```rust
// If expression
let number = if condition { 5 } else { 6 };

// For loop
for element in [10, 20, 30] {
    println!("value: {element}");
}

// Loop with label
'counting: loop {
    // ...
    break 'counting; // Breaks the labeled loop
}
```

## Functions
- Declared with `fn`
- Parameters require type annotations
- Return type specified with `->`
- The last expression in a function is implicitly returned (no semicolon)

### Example:
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon means this is the return value
}
```

## Integer Types
| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128-bit| i128   | u128     |
| Arch   | isize  | usize    |

## Modules and Code Organization

### Modules (`mod`)
- `mod` is used to define a module, which is a namespace for items like functions, structs, and other modules
- Modules help organize code into logical units and control visibility
- By default, items in a module are private to their parent module

### Function Visibility (`pub fn`)
- `pub` makes an item (like a function) accessible from outside its module
- Without `pub`, the item is private to its containing module
- Use `pub` to expose functionality that other parts of your program need to use

### Importing Modules
- Use `mod module_name;` to declare and include a module from a separate file
- The module name must match the file name (without .rs extension)
- After declaring a module, access its contents using the module name and `::` (e.g., `module_name::function_name`)
- You can also use `use` to bring items into scope

## Running the Code

To run the examples:

```bash
cargo run
```

Each concept is demonstrated in its respective module in the source files. The project structure is organized as follows:

- `src/main.rs`: Entry point of the application, imports and uses other modules
- `src/variables.rs`: Demonstrates variable declarations and shadowing
- `src/data_types.rs`: Shows different data types and their usage
- `src/control_flow.rs`: Contains examples of control flow structures
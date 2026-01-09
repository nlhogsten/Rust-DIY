# Rust Enums Tutorial

## What are Enums?

Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values.

## Enums vs Structs

**Structs** group related data together:
- All fields have the same structure
- Every instance has the same fields
- Example: `struct Person { name: String, age: u32 }`

**Enums** represent one of several possible variants:
- Each variant can have different types of data
- An enum value can only be one variant at a time
- Example: `enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }`

## Why Rust Doesn't Have Null

Rust doesn't have null because null references cause billions of dollars in errors and bugs (the "billion-dollar mistake"). Instead, Rust uses the `Option<T>` enum:

```rust
enum Option<T> {
    None,    // No value
    Some(T), // Some value of type T
}
```

This forces you to handle the case where there might be no value, eliminating null pointer exceptions.

## What are Options and Sum Types?

**Option<T>** is Rust's way to handle values that might be absent:
- `Some(value)` - contains a value
- `None` - represents no value

**Sum types** (like enums) are types that can be one of several different types. This is different from "product types" (like structs) which combine multiple fields into one type.

## How if let Works

`if let` is a concise way to handle one specific pattern match:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

This is shorthand for:
```rust
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    None => (),
}
```

## How match Works and How It Differs from if

**match** is Rust's pattern matching construct:
- **Exhaustive**: Must handle all possible cases
- **Powerful**: Can match on patterns, not just values
- **Returns values**: Each arm can return a value

```rust
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```

**if** statements are simpler conditional checks:
- Not exhaustive
- Only checks boolean conditions
- Doesn't return values (unless using if expressions)

```rust
if x > 0 {
    println!("positive");
} else if x < 0 {
    println!("negative");
} else {
    println!("zero");
}
```

## Running the Examples

To run all the enum examples:
```bash
cargo run
```

This will demonstrate:
1. Two ways to structure enums with data
2. Struct vs enum comparisons
3. if let pattern matching
4. match expressions
5. Option<T> usage
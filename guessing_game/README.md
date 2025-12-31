# Rust Guessing Game

This is a simple number guessing game that demonstrates several key Rust programming concepts.

## Key Rust Concepts Demonstrated

### 1. Module System and Imports
- `use std::io;` - Imports the `io` module from the standard library
- `use std::cmp::Ordering;` - Imports the `Ordering` enum from the `cmp` module
- `use rand::Rng;` - Imports the `Rng` trait from the `rand` crate

### 2. Syntax Elements
- `::` (double colon) - Used for:
  - Accessing associated functions (like `String::new()`)
  - Accessing items within modules (like `std::io`)
- `:` (colon) - Used for:
  - Type annotations (e.g., `let guess: u32`)
  - Trait bounds
  - Struct field definitions
  - Match arms

### 3. Types and Traits
- **`Ordering`**: An enum with variants `Less`, `Greater`, and `Equal` used for comparisons
- **`Rng`**: A trait that defines random number generation methods
- **`io`**: A module (lowercase) that provides I/O functionality

### 4. Error Handling
- `Result` type: Used for operations that can fail
- `expect()`: Unwraps a `Result`, panicking with a message on error
- `match`: Used for pattern matching and handling different outcomes

### 5. Memory Management
- `let` - Binds values to variables (immutable by default)
- `let mut` - Creates a mutable variable binding
- References (`&`) - Allow borrowing values without taking ownership

### 6. Common Patterns
- Shadowing: Reusing a variable name with `let`
- Pattern matching with `match`
- Loop control with `loop` and `break`

## Running the Game

1. Ensure you have Rust installed (install from [rustup.rs](https://rustup.rs/))
2. Run the game:
   ```bash
   cargo run
   ```
3. Enter your guess when prompted

## Dependencies

- `rand` - For generating random numbers

To view documentation for dependencies:
```bash
cargo doc --open
```

## Common Commands

- `cargo build` - Build the project
- `cargo run` - Build and run the project
- `cargo check` - Check code for errors without building
- `cargo update` - Update dependencies to latest compatible versions
- `cargo test` - Run tests
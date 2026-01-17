# Rust Packages, Crates, and Modules

This project demonstrates Rust's module system, which helps you manage code organization, control visibility, and structure your projects effectively.

## Overview

Rust's module system consists of several key components:

- **Packages**: Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules**: Control organization, scope, and privacy of paths
- **Paths**: Way of naming items like structs, functions, or modules

## Project Structure

This is a library crate that demonstrates module organization:

```
packages_crates_modules/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs
```

## Module Tree

The current module structure in this project:

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## Key Concepts

### Crates

- **Binary crates**: Contain a `main` function and compile to an executable
- **Library crates**: Don't have a `main` function and compile to a library
- **Crate root**: The file that the Rust compiler starts from (`src/main.rs` or `src/lib.rs`)

### Packages

A package is a collection of related crates. Cargo determines the crate type based on the files present:

- `src/lib.rs` → Library crate with the same name as the package
- `src/main.rs` → Binary crate with the same name as the package
- Both files → Two crates (binary and library) with the same name

### Module Visibility

- Modules are private by default
- Use `pub` keyword to make items public
- Parent modules can't access private child modules
- Sibling modules can access each other's public items

## Usage Examples

### Accessing Module Items

```rust
// Using the full path
packages_crates_modules::front_of_house::hosting::add_to_waitlist();

// Using use declarations
use packages_crates_modules::front_of_house::hosting;
hosting::add_to_waitlist();

// Bringing specific items into scope
use packages_crates_modules::front_of_house::hosting::add_to_waitlist;
add_to_waitlist();
```

### Creating Your Own Modules

```rust
pub mod my_module {
    pub fn public_function() {
        println!("This is public");
    }
    
    fn private_function() {
        println!("This is private to the module");
    }
}
```

## Building and Testing

```bash
# Build the library
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Best Practices

1. **Keep modules focused**: Each module should have a single responsibility
2. **Use privacy wisely**: Only expose what's necessary
3. **Organize logically**: Group related functionality together
4. **Use descriptive names**: Make your module structure self-documenting
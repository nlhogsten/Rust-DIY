# Rust Ownership System

This repository contains examples and explanations of Rust's ownership system, which is a key feature that ensures memory safety without needing a garbage collector.

## Memory Management in Rust

### The Stack
- **LIFO (Last In, First Out)** data structure
- Fast access (just push/pop operations)
- Stores fixed-size data types
- All data on the stack must have a known, fixed size at compile time

### The Heap
- Stores data with dynamic size or unknown size at compile time
- Slower than stack (requires allocation and deallocation)
- Access is done through pointers (memory addresses)
- Managed by Rust's ownership system

## Ownership Rules
1. Each value in Rust has a variable that's called its *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Key Concepts

### Moving and Copying
- **Move Semantics**: By default, assignment of values between variables moves ownership
- **Copy Trait**: Types that implement the `Copy` trait are copied instead of moved (e.g., integers, booleans, characters)
- **Clone Trait**: Explicit deep copy when needed

### References and Borrowing
- **References** allow you to refer to some value without taking ownership of it
- Multiple immutable references are allowed
- Only one mutable reference is allowed at a time
- References must always be valid

## Slices
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. They are a kind of reference, so they don't have ownership.

### String Slices
- A reference to part of a `String`
- Type is written as `&str`
- Immutable by default
- Created using range syntax: `&string[start..end]`

### Array Slices
- Similar to string slices but for arrays/vectors
- Type is written as `&[T]` where T is the type of elements
- Useful for working with portions of arrays/vectors

## Examples
Check the source files for practical examples of these concepts in action:
- `copy_and_move.rs`: Demonstrates move and copy semantics
- `references.rs`: Shows how borrowing works
- `slices.rs`: Contains examples of working with string and array slices
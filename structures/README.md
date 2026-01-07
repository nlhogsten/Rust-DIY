# Rust Structures Guide for Beginners

Structs allow you to bundle related data together, making your code more organized and readable.

## What are Structs?

Structs are custom data types that let you group related values together. Think of them as templates for creating complex data structures.

## Types of Structs

### 1. Classic Structs (Field Structs)
```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```
- **Use when**: You want named fields for clarity
- **Advantage**: Self-documenting code - each field has a meaningful name
- **Access**: `user.email`, `user.username`

### 2. Tuple Structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```
- **Use when**: You want to group data but don't need field names
- **Advantage**: More concise than classic structs
- **Access**: `my_color.0`, `my_color.1`, etc.
- **Key difference**: Same types can have different meanings (Color vs Point)

### 3. Unit-Like Structs
```rust
struct AlwaysEqual;
```
- **Use when**: You need a type but don't store any data
- **Advantage**: Useful for implementing traits on a type

## Methods

Methods are functions that belong to your struct, defined in an `impl` block.

### Understanding `self`

Think of `self` as "the thing I'm working on" - it refers to the specific instance of the struct that called the method.

```rust
impl Rectangle {
    // Instance method - works on a specific rectangle
    fn area(&self) -> u32 {
        // self is the rectangle that called .area()
        self.width * self.height  // access its own width and height
    }
    
    // Associated function - creates new rectangles
    fn square(size: u32) -> Self {
        Self {  // Self means "Rectangle" in this context
            width: size,
            height: size,
        }
    }
}

// Usage:
let rect = Rectangle { width: 30, height: 50 };
let area = rect.area();  // self = rect inside the method
let square = Rectangle::square(5);  // no self - creates new one
```

### Types of `self`:
- `&self`: Read-only access (can't change the struct)
- `&mut self`: Can modify the struct
- `self`: Takes ownership (rare, consumes the struct)

### Key Points:
- **Instance methods**: Use `self` to work on existing instances
- **Associated functions**: No `self`, often create new instances
- **Self vs self**: `Self` (capital S) = the type, `self` (lowercase) = the instance

## Important Attributes Explained

### `#[derive(Debug)]`
This automatically implements the `Debug` trait for your struct, allowing you to:
- Print your struct with `println!("{:?}", my_struct);`
- Use `dbg!(&my_struct);` for debugging
- Get helpful error messages

**Without it**: You can't print structs for debugging!

### `#[allow(dead_code)]`
This suppresses compiler warnings about unused fields or functions:
- Useful for learning code where not everything is used
- Tells Rust "I know this isn't used, it's intentional"
- Can be applied to individual items or entire structs

## What Are Traits?

Traits are like "abilities" or "interfaces" that you can give to your types. Think of them as labels that say "this type can do X".

Common traits:
- `Debug`: "This type can be printed for debugging"
- `Display`: "This type can be shown to users"
- `Clone`: "This type can be copied"

You add traits to your types with `impl` or `#[derive()]`.

## Debug Printing: `{:?}` vs `{}`

- `{}`: Requires `Display` trait (you must implement it yourself)
- `{:?}`: Requires `Debug` trait (auto-derived with `#[derive(Debug)]`)
- `{:#?}`: Pretty-print version of `{:?}` (better formatting)

### Should you remove `{:?}` in production?

**No!** Here's why:

1. **Debug vs Display**: They serve different purposes
   - `Debug`: For developers, shows internal structure
   - `Display`: For end users, shows user-friendly format

2. **Why not give everything Debug?**
   - **You should!** Most types benefit from Debug
   - Debug is lightweight and useful for maintenance
   - Only skip it for types that truly shouldn't be printed

3. **When to implement Display:**
   - When you need user-friendly output
   - When Debug format is too technical
   - For public APIs/end-user applications

**Rule of thumb**: Always derive `Debug`, implement `Display` only when needed for user-facing output.

## When to Use Each Type

| Situation | Best Choice | Why |
|-----------|------------|-----|
| User profile with named fields | Classic Struct | Clear, self-documenting |
| RGB color values | Tuple Struct | Concise, order matters |
| Marker type for traits | Unit-Like Struct | No data needed |
| Operations on data | Methods | Organized, object-oriented style |

## Learning Tips

1. Start with classic structs - they're most intuitive
2. Use `#[derive(Debug)]` on every struct while learning
3. Practice converting between different struct types
4. Methods make your code more organized than separate functions
# Hello Cargo

A simple Rust application demonstrating basic Rust syntax and Cargo usage.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/hello_cargo.git
   cd hello_cargo
   ```

2. Build the project:
   ```bash
   cargo build
   ```

### Running the Application

To run the program:
```bash
cargo run
```

This will compile and execute the program, which will print:
```
Hello, world!
```

## Project Structure

- `src/main.rs`: The main entry point of the application
- `Cargo.toml`: Project manifest containing dependencies and metadata

## Key Concepts Demonstrated

- **Main Function**: The `main` function is the entry point of every Rust program
- **Macros**: The `println!` macro is used for printing to the console
- **Comments**: Both line (`//`) and block (`/* */`) comments are supported

## Building for Release

To create an optimized release build:
```bash
cargo build --release
```

The optimized binary will be available at `target/release/hello_cargo`.

## Dependencies

This project uses the Rust standard library and has no external dependencies.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
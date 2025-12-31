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
   
   This command compiles your Rust code into an executable binary. By default, it creates a debug build which includes debugging information and is not optimized for performance. The resulting binary will be in `target/debug/hello_cargo`.

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
- `Cargo.toml`: Project manifest (similar to `package.json` in Node.js) containing:
  - Project metadata (name, version, authors)
  - Dependencies (similar to `dependencies` in package.json)
  - Build configurations
  - Other project settings
- `Cargo.lock`: Auto-generated dependency lock file (similar to `package-lock.json` or `yarn.lock` in Node.js)
- `target/`: Build directory containing:
  - Compiled binaries
  - Intermediate build artifacts
  - Dependency crates (similar to `node_modules` but managed differently)
  - Debug/Release builds (in `target/debug/` and `target/release/` respectively)

## Key Concepts Demonstrated

- **Main Function**: The `main` function is the entry point of every Rust program
- **Macros**: The `println!` macro is used for printing to the console
- **Comments**: Both line (`//`) and block (`/* */`) comments are supported

## Building for Release

To create an optimized release build:
```bash
cargo build --release
```

This creates a release build with optimizations enabled, making your program run faster but taking longer to compile. The optimized binary will be available at `target/release/hello_cargo`. Use this when you're ready to distribute your application or run performance benchmarks.

## Compiling with rustc

While Cargo is the recommended way to build Rust projects, you can also compile individual files directly using `rustc`:

```bash
rustc src/main.rs
./main
```

`rustc` is Rust's compiler. It takes a Rust source file and produces an executable. However, for most projects, using Cargo is preferred because it handles dependencies, project structure, and build configurations automatically. The main differences are:

- `rustc` is a low-level tool that compiles single files
- `cargo` is a higher-level tool that manages entire projects
- `cargo build` automatically handles dependencies and compiles all necessary files
- `cargo run` compiles (if needed) and runs your program in one step

## Dependencies and Build System

### Cargo.toml vs Cargo.lock

- **Cargo.toml** (like `package.json` in Node.js):
  - Human-editable manifest file
  - Declares your project's metadata and dependencies
  - Uses semantic versioning for dependencies (e.g., `serde = "1.0"`)
  - Specifies build configurations and features

- **Cargo.lock** (like `package-lock.json` or `yarn.lock` in Node.js):
  - Auto-generated file (do not edit manually)
  - Locks exact versions of all dependencies and their dependencies
  - Ensures reproducible builds across different machines
  - Committed to version control for applications (but not for libraries)

### The `target` Directory

The `target` directory is where Cargo stores all build artifacts and intermediate files:
- `target/debug/`: Contains unoptimized development builds
- `target/release/`: Contains optimized release builds
- `target/debug/deps/`: Compiled dependencies
- `target/debug/build/`: Build script output

Unlike Node.js's `node_modules`, the `target` directory can be safely deleted and will be regenerated on the next build. It's typically added to `.gitignore` for libraries (but often committed for applications to speed up CI/CD).

This project currently uses only the Rust standard library and has no external dependencies.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
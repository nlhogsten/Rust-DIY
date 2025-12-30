// main functions are always the first to run in a rust program
fn main() {
// parameters go inside (), function body in {}
    println!("Hello, world!"); // end with semicolon
    // println! is a rust macro where '!' indicates it's a macro rather than a regular function
}

/*
Run this file with: rustc main.rs && ./main
The first command compiles the Rust code into an executable.
The second command runs the executable.
Notice the plain 'main' file, this is runnable even if rust isn't installed globally - it's self-contained.
*/
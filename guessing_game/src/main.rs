/*
    Bring in 'io' input/output functionality from the std library
    Standard default library in the Rust called the 'prelude'
*/
use std::io;

// fn declare a function with input '()' and body '{'
fn main() {
    println!("Guess the Number!");

    println!("Please input your guess.");

    /*
        Declare a mutable string variable
        Variables are immutable by default
        let apples = 5; (immutable)
        let mut bananas = 5; (mutable)
    */
    let mut guess = String::new();
    /*
        Create an empty string instance
        :: syntax for calling associated functions
        String::new() is an associated function that creates a new empty string
        It's called on the String type itself, not on an instance
        This is how we initialize a new, empty string to store user input
        The variable 'guess' will hold the user's input as a string
    */
    
    /*
        Call input functionality from std library
        'std::io::stdin' could have been used here without an import
    */
    io::stdin()
        .read_line(&mut guess) // 'read_line' accepts user input, '&' is a reference to variable
        .expect("Failed to read line.");
    /* 
        This returns a Result type that needs to be handled.
        The Result type is an enum that can be either Ok(value) or Err(error).
        We use .expect() to panic and exit if there's an error.
    */

    println!("You guessed: {guess}");
    /*
        Brackets are used for string interpolation in Rust.
        The curly braces {} are replaced with the value of the variable.
        let x = 5;
        let y = 10; 
        println!("x = {x} and y + 2 = {}", y + 2); prints 'x = 5 and y + 2 = 12'
        This is similar to f-strings in Python or template literals in JavaScript.
    */
}

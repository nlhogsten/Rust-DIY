/// Demonstrates Rust's built-in Option<T> enum
/// Option is used when a value might be present or absent
/// This is Rust's way of handling null values safely
pub fn option_demo() {
    println!("=== Option Examples ===");
    
    // Option<T> is built into Rust and has two variants:
    // Some(T) - contains a value of type T
    // None - represents no value
    
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some char: {:?}", some_char);
    println!("Absent number: {:?}", absent_number);
    
    // Using match to handle Option
    fn print_number(x: Option<i32>) {
        match x {
            Some(num) => println!("Number is: {}", num),
            None => println!("No number provided"),
        }
    }
    
    print_number(some_number);
    print_number(absent_number);
    
    // Using if let for simpler cases
    if let Some(num) = some_number {
        println!("Found number with if let: {}", num);
    }
}

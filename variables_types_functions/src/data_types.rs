use std::io;

pub fn demonstrate_scalar_data_types() {
    // Rust is a statically typed language, but the compiler can usually infer types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {}", x);
    println!("y = {}", y);

    // Integer operations
    // addition
    let sum = 5 + 10;
    println!("sum = {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("product = {}", product);
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("truncated = {}", truncated);
    println!("quotient = {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("remainder = {}", remainder);

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {}", t);
    println!("f = {}", f);

    // Characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c = {}", c);
    println!("z = {}", z);
    println!("heart_eyed_cat = {}", heart_eyed_cat);
}

pub fn demonstrate_compound_data_types() {
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    // The :? is the debug formatter. It's used to print values for debugging purposes.
    // It requires the type to implement the Debug trait (which most standard types do).
    // This is different from {} which requires the Display trait.
    // The :? is especially useful for complex types like tuples, structs, and enums.
    println!("tup = {:?}", tup);

    // Create a tuple with three different data types
    let tup = (500, 6.4, 1);
    // Destructure the tuple into individual variables
    // The pattern (x, y, z) matches the structure of the tuple
    // and assigns each value to the corresponding variable
    let (x, y, z) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    // Access tuple elements by index
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred = {}", five_hundred);
    println!("six_point_four = {}", six_point_four);
    println!("one = {}", one);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    let a: [i32; 5] = [6, 7, 8, 9, 10];
    println!("a = {:?}", a);
    let first = a[0];
    let second = a[1];
    println!("first = {}", first);
    println!("second = {}", second);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
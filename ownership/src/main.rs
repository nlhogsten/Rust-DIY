mod copy_and_move;
mod references;
mod slices;

fn main() {
    string_type();
    string_literal();
    scoped_variables();
    deep_clone();
    copy_and_move::demonstrate_copy_and_move();
    references::references_demo();
    slices::test_slices();
}

// variable 's' goes out of scope and is dropped
fn string_type() { 
    let mut s = String::from("hello");
    s.push_str(" string type!"); // push_str() appends a literal to a String
    println!("{s}"); // this will print `hello string type!`
}
// variable 's' goes out of scope and is dropped

fn string_literal() { 
    let s = String::from("hello string literal!");
    println!("{s}");
}

/* 
    String literals are stored in the binary itself and are immutable.
    String types are mutable and allocated on the heap.
*/

fn scoped_variables() {
    let s1 = String::from("s1 was borrowed by s2!");
    let s2 = s1;

    // This will cause a compile error because s1 is moved to s2
    // println!("{s1}, world!");
    
    // s2 is now the owner of the string data
    println!("{s2}");
}

fn deep_clone() {
    // If we don't want to drop the value, we can clone it
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    
    // Integer types are on the stack, so they don't need to be cloned
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

}

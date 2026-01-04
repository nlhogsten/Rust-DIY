pub fn references_demo() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");

    let mut s = String::from("hello");
    change_reference(&mut s);
    println!("Modified string: {s}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()

    // s.push_str("!");
    // cannot modify a reference
} // Here, s goes out of scope. But because s does not have ownership of what it refers to, the String is not dropped.

fn change_reference(some_string: &mut String) {
    some_string.push_str(", world");
}
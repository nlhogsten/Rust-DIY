//! Examples demonstrating the use of slices in Rust.
//! 
//! Slices let you reference a contiguous sequence of elements in a collection
//! rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

/// Demonstrates various ways to work with string and array slices
pub fn test_slices() {
    // String slice example
    let my_string = String::from("hello world");
    
    // Get first word using our custom function
    let word = first_word(&my_string);
    println!("First word: '{}'", word);
    
    // String literals are slices
    let s = "Hello, world!";  // s is of type &str (string slice)
    println!("String literal: {}", s);
    
    // Array slice example
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];  // Gets elements at index 1, 2, and 3
    println!("Array slice: {:?}", slice);
    
    // More string slice examples
    let s = String::from("hello world");
    
    // These are equivalent
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Sliced: '{}' and '{}'", hello, world);
    
    // You can omit start or end to use the start/end of the string
    let start = &s[..5];  // Same as [0..5]
    let end = &s[6..];    // From index 6 to end
    println!("Partial slices: '{}' and '{}'", start, end);
    
    // Get slice of entire string
    let whole = &s[..];
    println!("Whole string as slice: '{}'", whole);
}

/// Returns the first word in a string, or the whole string if no space is found
/// 
/// # Arguments
/// * `s` - A reference to a String
/// 
/// # Returns
/// A string slice (&str) containing the first word or the whole string
fn first_word(s: &str) -> &str {
    // Convert the String to a byte array
    let bytes = s.as_bytes();

    // Iterate through the bytes, getting both index and value
    for (i, &item) in bytes.iter().enumerate() {
        // If we find a space, return the slice up to this index
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If no space is found, return the whole string
    &s[..]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }
    
    #[test]
    fn test_array_slices() {
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
    }
}

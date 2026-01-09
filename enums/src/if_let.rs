/// Demonstrates the if let control flow construct
/// if let is a concise way to handle one specific pattern match
pub fn if_let_demo() {
    // Option represents a value that might be present or absent
    let config_max = Some(3u8);
    
    // if let lets us handle the Some case without a full match
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    
    // Example with None value
    let config_none: Option<u8> = None;
    
    if let Some(max) = config_none {
        println!("This won't print: {max}");
    } else {
        println!("No maximum configured");
    }
}

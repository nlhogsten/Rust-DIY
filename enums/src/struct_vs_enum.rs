/// Demonstrates the difference between enums and structs
/// Enums can group different types of data together in one type,
/// while structs require separate types for each variant.
pub fn struct_vs_enum() {
    // Enum: One type that can be any of these variants
    #[allow(dead_code)]
    enum Message {
        Quit,                                    // No data
        Move { x: i32, y: i32 },                 // Anonymous struct
        Write(String),                           // Single string value
        ChangeColor(i32, i32, i32),              // Three numeric values
    }

    // Equivalent structs: Each is a separate type
    #[allow(dead_code)]
    struct QuitMessage; // unit struct - no fields
    #[allow(dead_code)]
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    #[allow(dead_code)]
    struct WriteMessage(String); // tuple struct - one unnamed field
    #[allow(dead_code)]
    struct ChangeColorMessage(i32, i32, i32); // tuple struct - three unnamed fields

    // Enums can have methods implemented on them
    impl Message {
        fn call(&self) {
            // method body would be defined here
            println!("Called message processing");
        }
    }

    // Create instances of each enum variant to demonstrate usage
    let quit_msg = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("hello"));
    let color_msg = Message::ChangeColor(255, 0, 0);
    
    // Call methods on each
    quit_msg.call();
    move_msg.call();
    write_msg.call();
    color_msg.call();
    
    println!("Created all message types");
}
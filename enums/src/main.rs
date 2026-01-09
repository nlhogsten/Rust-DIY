mod struct_vs_enum;
mod if_let;
mod match_examples;
mod options;

fn main() {
    println!("=== Rust Enums Tutorial ===\n");
    
    // Run all enum demonstrations
    method_one();
    method_two();
    struct_vs_enum::struct_vs_enum();
    if_let::if_let_demo();
    match_examples::match_demo();
    options::option_demo();
}

/// Method 1: Enum + Struct pattern
/// Shows how to use enums with structs to associate data
fn method_one() {
    println!("=== Method 1: Enum + Struct ===");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    println!("Home: {:?}, Loopback: {:?}", home, loopback);
    println!();
}

/// Method 2: Enum with data directly
/// Shows how to embed data directly in enum variants
fn method_two() {
    println!("=== Method 2: Enum with Embedded Data ===");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Home: {:?}, Loopback: {:?}", home, loopback);
    println!();
}



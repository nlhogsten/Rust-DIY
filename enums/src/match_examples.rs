/// Demonstrates the match control flow operator
/// match is exhaustive - it must handle all possible values
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/// Returns the value of a coin in cents
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/// Demonstrates matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/// Runs the match examples
pub fn match_demo() {
    println!("=== Match Examples ===");
    
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    
    println!("Penny value: {} cents", value_in_cents(penny));
    println!("Nickel value: {} cents", value_in_cents(nickel));
    println!("Dime value: {} cents", value_in_cents(dime));
    println!("Quarter value: {} cents", value_in_cents(quarter));
    
    // Option examples
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("Five plus one: {:?}", six);
    println!("None plus one: {:?}", none);
}

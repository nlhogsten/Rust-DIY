// Import the module containing our demonstrations
mod variables;
mod data_types;
mod control_flow;

fn main() {
    // Call the function from our modules
    control_flow::control_flow();
    control_flow::loop_labels();
    control_flow::while_loop();
    control_flow::for_loop();

    new_function(5, 'h');
    statements_and_expressions();

    let x = return_function(5);
    println!("The value of return_function is: {x}");

    variables::demonstrate_variables();
    data_types::demonstrate_scalar_data_types();
    data_types::demonstrate_compound_data_types();
}

fn new_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn statements_and_expressions() {
    // Statement
    let y = 6;
    println!("The value of y is: {y}");
    // Expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn return_function(x: i32) -> i32 {
    x + 1
    // No semicolon after the return value
    // Semicolon means it's a statement, not an expression
    // Expressions evaluate to a value and don't include a semicolon
}

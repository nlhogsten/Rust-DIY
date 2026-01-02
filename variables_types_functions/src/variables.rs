pub fn demonstrate_variables() {
    // Constants are immutable by default, all caps naming convention
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant value: {THREE_HOURS_IN_SECONDS}");

    /*
        Variables are immutable by default
        Shadowing is allowed by redeclaring the 'let' variable
    */
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the parent scope is: {x}");

    /*
        ** Can change between types with same variable using shadowing
        let spaces = "   ";
        let spaces = spaces.len();

        ** This throws compile error because we're trying to mutate an mut variable's type
        let mut spaces = "   ";
        spaces = spaces.len();
    */
}

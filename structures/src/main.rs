mod find_area;
mod methods;

fn main() {
    // create a user
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{:?}", user1);
    // update that user
    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1);
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // fills out remaining fields with values from user1
    };
    println!("{:?}", user2);
    // create a user using a function
    let user3 = build_user(String::from("thirduser@email.com"), String::from("thirdusername"));
    println!("{:?}", user3);

    find_area::struct_version();
    find_area::tuple_version();
    find_area::variable_version();

    methods::methods();
}

#[derive(Debug)]
#[allow(dead_code)]
struct User { // struct name describes significance of data being together
    active: bool, // each data point is a 'field': key:value pair
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}



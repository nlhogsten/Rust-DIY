pub fn variable_version() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_variables(width1, height1)
    );
}

pub fn tuple_version() {
    let rect1 = (3, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(rect1)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn struct_version() {
    let rect1 = Rectangle {
        width: 30,
        height: 5,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_structs(&rect1)
    );
    println!("rect1 is {rect1:?}");
    dbg!(&rect1);
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_variables(width: u32, height: u32) -> u32 {
    width * height
}
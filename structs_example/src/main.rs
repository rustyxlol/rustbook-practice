// write a program that calculates the area of a rectangle.
// We’ll start by using single variables, and then refactor the program until
// we’re using structs instead.

// Normal way
// ***********************
// fn main() {
//     let l = 10;
//     let b = 20;

//     let a = area(l, b); // makes a copy of l and b so no need to borrow

//     println!("Area = {}", a);
// }

// fn area(l: u32, b: u32) -> u32 {
//     l * b
// }

// Tuple way
// ***********************

// fn main() {
//     let rect: (u32, u32) = (10, 20);

//     let a = area(&rect); // primitive types so ownership isn't moved, but do it anyway

//     println!("{}{}", rect.0, rect.1);
//     println!("Area = {}", a);
// }

// fn area(dimensions: &(u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Using struct way 1
// ***********************

// struct Rectangle(u32, u32);

// fn main() {
//     let rect1 = Rectangle(10, 20);

//     let a = area(&rect1); // don't transfer ownership

//     println!("{} {}", rect1.0, rect1.1);
//     println!("Area is: {}", a);
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.0 * rect.1
// }

// Using structs 2
// ***********************
//debug attribute to print Rectangle
#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

fn main() {
    // let scale = 2;
    let rect1 = Rectangle {
        length: 20,
        breadth: 12,
        // breadth: dbg!(30 * scale),
        // note that dbg! returns ownership so breadth will be 30 * scale
    };

    // dbg!(&rect1); // debug macro

    println!("Rectangle is {:#?}", rect1);

    let a = area(&rect1); // need to borrow because we don't want to transfer ownership

    println!("Area is: {}", a);
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.breadth
}

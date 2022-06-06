/*
 Object structure
 It should contain:
 1. Shape(enum)
 2. Color(struct)
 3. Origin(struct)
*/
use std::io;

#[derive(Debug)]
struct Object {
    shape: Shape,
    color: Color,
    location: Origin,
}

impl Object {
    fn new(shape: Shape, color: Color, location: Origin) -> Object {
        Object {
            shape,
            color,
            location,
        }
    }
}
#[derive(Debug)]
enum Shape{
    Square,
    Rectangle,
    Circle,
    Triangle,
    Polygon,
    Ellipse,
}

impl Shape {
    fn get_shape(number: &str) -> Shape {
        match number {
            "1" => Shape::Square,
            "2" => Shape::Rectangle,
            "3" => Shape::Triangle,
            "4" => Shape::Circle,
            "5" => Shape::Ellipse,
            "6" => Shape::Polygon,
            _ => Shape::Square,
        }
    }
}
#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
#[derive(Debug)]
struct Origin {
    x: u8,
    y: u8,
}

fn main() {
    let obj: Object = Object::new(Shape::Circle, Color {r: 20, g: 30, b: 30}, Origin {x: 20, y: 0});

    println!("{:#?}", obj);

}


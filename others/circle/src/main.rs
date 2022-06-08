use std::io;
use std::f64::consts::PI;

struct Circle {
    radius: f64
}

impl Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2f64 * PI * self.radius
    }
}

fn get_radius() -> f64 {
    let mut radius = String::new();

    io::stdin()
    .read_line(&mut radius)
    .expect("Error reading");

    let radius: f64 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => 0f64,
    };

    radius
}

fn main() {
    let circle: Circle = Circle {radius: get_radius()};
    println!("Area: {}", circle.area());
    println!("Perimeter: {}", circle.circumference());
}

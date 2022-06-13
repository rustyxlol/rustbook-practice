use std::env;
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


fn main() {
    let args: Vec<String> = env::args().collect();

    let radius = &args[1];
 
    let radius: f64 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a valid number")
    };

    let circle: Circle = Circle {radius};

    println!("The radius is: {}", radius);
    println!("The circumference is: {:.2}", circle.circumference());
    println!("The area is: {:.2}", circle.area());
}

fn main() {
    println!("Hello");
    another_function(20, '1');
    let fahrenheit = 100.0;
    let celsius = get_celsius(fahrenheit);

    println!("{} fahrenheit is {} celsius", fahrenheit, celsius);
}

fn another_function(x: u32, y: char) {
    println!("{} {}", x, y);
}

fn get_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 0.55 // if no semi-colon then it's an expression and has return value
}

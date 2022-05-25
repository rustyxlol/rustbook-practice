use std::io;
use std::io::Write;

fn main() {
    println!("Enter conversion");
    println!("1. Convert celsius to fahrenheit");
    println!("2. Convert fahrenheit to celsius");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Enter a number");

    let choice: u32 = choice.trim().parse().expect("Error");

    let mut temperature = String::new();

    if choice == 1 {
        println!("Enter celsius: ");
    } else {
        println!("Enter fahrenheit: ");
    }

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temperature).expect("Okay!");

    if choice == 1 {
        let temperature: f64 = temperature.trim().parse().expect("Error");
        let converted_temperature = c_to_f(temperature);

        println!("{}°c is {}°f", temperature, converted_temperature);
    } else if choice == 2 {
        let temperature: f64 = temperature.trim().parse().expect("Error");
        let converted_temperature = f_to_c(temperature);

        println!("{}°f is {}°c", temperature, converted_temperature);
    } else {
        println!("Wrong choice boohoo");
    }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 0.555
}

fn c_to_f(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

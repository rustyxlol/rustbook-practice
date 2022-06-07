use std::env;
use std::str::FromStr;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"))
    }

    if numbers.len() == 0 {
        println!("USAGE: gcd NUM1 NUM2...");
        std::process::exit(1);
    }

    let mut result = numbers[0];

    for b in &numbers[1..] {
        result = gcd(result, *b);
    }

    println!("GCD of {:?} is: {}", numbers, result);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(270, 192), 6);
}

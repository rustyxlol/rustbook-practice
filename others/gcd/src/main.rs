use std::cmp::Ordering;
use std::io;
/**
 * A program to calculate GCD of two numbers (Euclid algorithm)
 * If A = 0 then GCD(A,B)=B, since the GCD(0,B)=B, and we can stop.
 * If B = 0 then GCD(A,B)=A, since the GCD(A,0)=A, and we can stop.
 * Write A in quotient remainder form (A = B⋅Q + R)
 * Find GCD(B,R) using the Euclidean Algorithm since GCD(A,B) = GCD(B,R)
 */

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_number() -> u64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading");

    let input: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    input
}

fn main() {
    println!("Enter first number: ");
    let input1 = get_number();

    println!("Enter second number: ");
    let input2 = get_number();

    // Can be done using if but this looked cleaner
    let result: u64 = match input1.cmp(&input2) {
        Ordering::Less => gcd(input2, input1),
        _ => gcd(input1, input2),
    };

    println!("GCD of {} and {} is: {}", input1, input2, result);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(270, 192), 6);
}
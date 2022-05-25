use std::io;
use std::io::Write;

fn main() {
    let mut n = String::new();

    println!("Enter nth fibonacci index: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut n).expect("Error reading");

    let n: u128 = n.trim().parse().expect("Error");

    println!("nth fibonacci term is: {}", nth_fibonacci(n));
}

fn nth_fibonacci(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}

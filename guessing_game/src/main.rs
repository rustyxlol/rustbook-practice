// The first part of the guessing game program will ask for user input, process
// that input, and check that the input is in the expected form

/* PRELUDE */
use rand::Rng;
use std::cmp::Ordering;
use std::io; // Standard library input/output
use std::io::Write; // For flush // for comparison

/* MAIN FUNCTION */
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap(); // IDK WHY you need this for single line print - input

        let mut guess = String::new();

        /*
        Variables are immutable by default in rust, hence the addition of
        mut keyword

        new is an associated function of String type, that's implemented on String(type)
        */

        io::stdin() // io contains stdin function, same as String contains new
            .read_line(&mut guess) // References are also immutable by default, and references are primarily used
            .expect("Failed to read line"); // Throw an error if an exception occurs
                                            /* read_line returns io::Result, an enum */
        /* io::Result has an except method, if Result is Ok then it contains value, else Err, an error occurs*/

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Secret number is higher"),
            Ordering::Greater => println!("Secret number is lesser"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything except for this function's signature
fn print_number(maybe_number: option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(some(13));
    print_number(some(99));

    let mut numbers: [option<u16>; 5] = [some(0); 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}

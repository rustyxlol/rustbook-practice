fn main() {
    let guess: u32 = "42".parse().expect("Error");
    println!("Guess is: {}", guess);

    let tup: (i32, f64) = (10, std::f64::consts::PI);
    // let tup2 = (10, 3.10);
    let (x, y) = tup;

    println!("Accessing after unpacking");
    println!("x = {}\ny = {}", x, y);
    println!("Accessing using indices with .");
    println!("x = {}\ny = {}", tup.0, tup.1);

    let a1 = [1, 2, 3, 4, 5, 6];

    // let a2: [i32; 5] = [1, 2, 3, 4, 5]; // type and number of elements

    // let a3 = [3; 5];

    println!("Array indexing: a1[0]: {}", a1[0])
}

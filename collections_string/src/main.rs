fn main() {
    let mut hello = String::from("السلام عليكم");
    let world = String::from("World");

    hello.push_str(": Hello");

    println!("{}", hello);

    let hw = hello + &world;

    for c in hw.chars() {
        println!("{}", c);
    }
}

// ******************************************************
// fn main() {
//     let x = 5;
//     let y = x;
//     // copy of x value is created and binded to y(x value, not x itself)
//     // x and y are pushed onto the stack because 1. fixed size 2. known value
//     println!("x={x}, y={y}");

//     let s1 = String::from("hello");
//     let s2 = s1; // s1 goes out of scope because s2 ptr points to s1[0]

//     println!("s1 = {s1}, s2={s2}"); // error
// }
// ******************************************************

// ******************************************************
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{s}"); // gives error, s is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
  // ******************************************************

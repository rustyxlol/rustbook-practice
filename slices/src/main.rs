// Here's a small programming problem: write a function that takes a
// string and returns the first word it finds in that string.
// If the function doesn't find a space in the string, the whole
// string must be one word, so the entire string should be returned.

// 1. No idea how to return word, try returning index of first space
// 2. Contemplate and figure out a way to use slices
// 3. Use slices
// 4. ????
// 5. Profit

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // we get a reference to element from .iter().enumerate() hence &bit
    for (i, &bit) in bytes.iter().enumerate() {
        if bit == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn main() {
    let s = String::from("Hello");

    let word = first_word(&s);

    println!("{word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

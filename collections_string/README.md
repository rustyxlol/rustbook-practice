Stuff covered  

1. Two strings cannot be added, left one needs to be a String(ownership is lost) while the other one needs to be a reference
2. You can use `format!("{}-{}-{}", s1, s2, s3);`, this way, the ownership is preserved unlike above.
3. Indexing is haram in Strings.
4. But slicing isn't, `&hello[0..4]`, but do be careful when slicing.
5. Two ways to iterate over strings
    ```rs
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    ```
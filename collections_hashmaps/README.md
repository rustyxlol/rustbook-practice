Stuff covered

1. Just like other collections, heap is used.
2. Two ways to populate Hashmaps
   1. `someHashMap.insert(String::from("Blue"), 10);`
   2. Combine two vectors like:
      ```rs
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let mut scores: HashMap<_, _> =
            teams.into_iter().zip(initial_scores.into_iter()).collect();
        ```
3. Ownership is lost upon insertion if variables are used.
4. Access using `.get(key)`, make sure to use match case because get returns an `Option`
5. You can also iterate over hashmaps like so:
    ```rs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    ```
6. `.insert` will overwrite if a key already exists.
7. `.entry` is used to check if a value already exists, if it doesn't `or_insert` can be used. `scores.entry(String::from("Blue")).or_insert(50);`
8. `or_insert` returns mutable value of the given key
Stuff covered  

1. There are two ways to access vectors
   1. using &v[index] - panic if index out of range
   2. using `v.get(index)` - need to use match to deal with index out of range
2. Be careful with references,  you cannot use immutable borrow if you modify a mutable vector
    ```rs
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);

        println!("The first element is: {}", first); // throws error
    ```
3. Two ways to iterate
   1. Immutable way - to read
        ```rs
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        ```
   2. Mutable way - to read and write  
        Remember dereferencing operator? Yeah, use that to get the value in i, because it is of the type `&mut {integer}`, dereferencing it will make it `{integer}`
        ```rs
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        ```
4. Vectors can feel inconvenient because they store values of the same type, a quick fix would be to use an enum and put whatever you want in there
    ```rs
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    ```

    
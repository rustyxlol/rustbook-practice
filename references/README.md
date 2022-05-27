Stuff covered

Just read the book.

Scenario: Imagine you are lending something to your imaginary friend
1. If you give X to a friend, and X is mutable, then there can exist only ONE reference of mutable X and MULTIPLE of immutable X
2. One mutable and one immutable references CANNOT exist!
3. There are no dangling pointers.
4. Creating of a reference is known as borrowing
5. Make sure to account for `Move` when referencing.  
   ```rs
   fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s 
    } 
    // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!

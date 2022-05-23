Stuff covered:  
1. variables can be redeclared and redefined when `mut` is used. 
```rs
let mut x = 32;
println!("x is {}", x);

let mut x = 42;
x = 69;
println!("x is {}", x); // x is 69
```
2. variables can be redeclared when `mut` is not used, but cannot be redefined
```rs
let x = 20;
let x = 30; // valid

println!("x is {}", x); // works

x = 20; // does not work!
```

3. variables can be shadowed, i.e. same variable can be used for different things
```rs
let x = 5; // x = 5

let x = x + 1; // x = 6

{
    let x = x * 2; // inner x = outer x * 2
    println!("x: {}", x); // inner x = 12
}

println!("x: {}", x); // x = 6
/********************************/

// Shadowing to reduce multiple variables
let spaces = "    ";
let spaces = spaces.len();

// compile time error if you use mut here
// because they aren't the same types
```

4. Constants don't support `mut`, nor they can be redeclared, nor redefined.


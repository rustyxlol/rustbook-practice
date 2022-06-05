Stuff covered

1. Enums allow you to define a type by enumerating its possible variants
2. Don't have to use structs for enums, enums can have types too!
```rs
// Don't do this!
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

// Do this 
enum IpAddrKind {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
```
3. It is not limited to just one data type, you can do `V4(u8, u8, u8, u8)` as well. Even structs, and EVEN ENUMS THEMSELVES!
4. `Option` enum is defined by the standard library by default
    1. Used for a scenario where a value could either be something or nothing
    ```rs
    enum Option<T> {
        None, 
        Some(T),
    }
    ```
5. In order to use Option<T>, you must first convert it to T, or else operations with <T> and T together won't work
6. Variants of Option must be handled if you're using Option. "You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T. You want some other code to run if you have a None value, and that code doesn’t have a T value available"  

TIP: Use match

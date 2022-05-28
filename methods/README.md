Stuff covered
1. Methods are kind of like functions, except they are defined within the context of a struct (or an enum or a trait object
2. All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl
3. Not all associated functions are methods
4. e.g. functions that do not require self(instance of a type)
5. Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct
```rs  
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let sq = Rectangle::square(3);
```
Getting a rectangle instance from 1 size

6. Multiple impl blocks can exist for the same struct.
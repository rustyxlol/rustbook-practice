Stuff covered



| Length  | Signed | Unsigned |
|-------|----------|----------|
|8-bit   |i8       |u8        |
|16-bit  |i16      |u16       |
|32-bit  |i32      |u32       |
|64-bit  |i64      |u64       |
|128-bit |i128     |u128      |
|arch    |isize    |usize     |


|Number literals |   Example   |
|----------------|-------------|
|Decimal         | 98_222      |
|Hex             | 0xff        |
|Octal           | 0o77        |
|Binary          | 0b1111_0000 |
|Byte (u8 only)  | b'A'        |


1. Type annotations are required since Rust is statically typed, e.g. `let guess: u32 = "42".parse().expect("not a number")`  
2. Tuples, same as python, unpacking is also the same
```rs
let tup:(i32, i32, f64) = (10, 20, 3.14);
let (x, y, z) = tup;
// x = 10
// y = 20
// z = 3.14
```
3. They can also be accessed by indexing using `.`. e.g., `tup.0` = `10`, `tup.1` = `20`
4. Arrays, must contain elements of same types and are represented using `[]` and is **allocated on the** **stack**, rather than the **heap**.
```rs
let a = [1, 2, 3, 4, 5];
// type can be specified like so
let b: [i32; 5] = [1, 2, 3, 4, 5];
// initialization is also possible using
let a = [3; 5];
// [3, 3, 3, 3, 3]
// Access using indexing
let y = a[0];
```
		
		
		
		
		
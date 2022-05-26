Stuff covered  
1. All data stored on the stack must have a known, fixed size
2. Data with an unknown size at compile time or a size that might change must be stored on the heap instead
3. Heap scenario: Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you
4. Ownership rules:
   1. Each value in Rust has a variable that’s called its owner.
   2. There can only be one owner at a time.
   3. When the owner goes out of scope, the value will be dropped.
5. Ways Variables and Data Interact:
   1. Move - scope of prior variable in heap is gone when one String is assigned to another(example)
   2. Clone - Heap data gets copied and both variables are not 'lost', eg. `s2 = s1.clone()`
   3. Stack-Only Data: Copy - `let x = 4` then `let y = x` is valid because the size is known at compile time and so they are kept in the stack, so the copy value operation is rather quick, hence no need to clone it. Scope is also maintained for both variables.
6. Any group of simple scalar values can implement Copy - a trait that doesn't require cloning or doesn't have `Drop` trait implemented with it, for example:
   1. All the integer types, such as u32.
   2. The Boolean type, bool, with values true and false.
   3. All the floating point types, such as f64.
   4. The character type, char.
   5. Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
7. Passing a variable to a function will move or copy, just as assignment does
8. Return values will also move or copy, just like assignment does.
9. remember: Assigning a value to another variable moves it. If the value was in heap, then it is cleaned up by `drop` after it goes out of scope or ownership is transferred to another variable.
10. This might be quite a pain to deal with, what if you wanted to pass a variable to a function without having to do all of the above? Rust introduces references. 
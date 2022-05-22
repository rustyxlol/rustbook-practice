This chapter covered:
1. Adding dependencies, you add them directly to `Cargo.toml` and build the project.
2. `Cargo.lock` file is for rebuilding, Rust won't have to search for versions
3. Bringing types into scope using `use` 
4. `match` functions like a `switch`
5. functions such as `read_line` or `parse` return `io::Result` which can either be `Ok` or `Err`, both contain values inside them. i.e., `Ok(num)` will return `num`. 
6. In `Err(_)`, `_` is catchall 
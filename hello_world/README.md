Commands covered:  

1. `rustc file_name.rs` to compile and run `file_name.rs`
2. `cargo new project_name` - Creates a project
3. `cargo build` - Builds the project binaries
4. `cargo run` - builds and runs, runs if binaries already exist
5. `cargo check` - Checks code for compilation, doesn't produce executables. It is also faster than cargo build most of the time.
6. `cargo build --release` builds the project for final release in target/release directory

Others:
1. Command followed by `!` represents a macro. e.g., `println!()`
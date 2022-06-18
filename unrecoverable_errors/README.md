Stuff covered  

1. You can do two things when dealing with unrecoverable errors
   1. Unwind the stack, Rust goes back and tries to free all the memory prior to the panic line. This requires some work.
   2. If you want to minimize the size of your binaries, you can use
        ```toml
        [profile.release]
        panic = 'abort'
        ```  
        in `Config.toml` and the OS will clean up the memory left.

2. Learn to read backtrace by using `RUST_BACKTRACE=1 cargo run`. Read from top to bottom and see which file is yours that caused the program to panic.
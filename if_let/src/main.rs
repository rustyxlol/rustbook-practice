fn main() {
    let config_max = Some(3u8);

    // The match way:
    // match config_max {
    //     Some(max) => println!("The max is {}", max),
    //     _ => (),
    // }

    // The if let way:
    if let Some(max) = config_max {
        println!("The max is {}", max)    
    }
}

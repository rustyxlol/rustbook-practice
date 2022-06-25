use minigrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(values) => values,
        Err(_) => {
            println!("Error occured while reading arguments");
            std::process::exit(1);
        }
    };

    println!(
        "Searching for {} In file {}",
        config.query, config.file_name
    );

    if let Err(e) = minigrep::run(config) {
        println!("Random error occured!");
        std::process::exit(1);
    }
}

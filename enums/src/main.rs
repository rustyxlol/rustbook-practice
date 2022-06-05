use std::io;

enum Direction{
    Up,
    Down,
    Left,
    Right,
    Wrong
}

impl Direction {
    fn get_dir(direction: &str) -> Direction{
        match direction.trim() {
            "w" => Direction::Up,
            "s" => Direction::Down,
            "a" => Direction::Left,
            "d" => Direction::Right,
            _ => Direction::Wrong,
        }
    }
}

fn main() {
    println!("Enter direction to move: ");
    loop {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
        direction_match(&input)
    } 
}

fn direction_match(direction: &str) {
    match Direction::get_dir(direction) {
        Direction::Up => println!("Player is going up!"),
        Direction::Down => println!("Player is going Down!"),
        Direction::Left => println!("Player is going Left!"),
        Direction::Right => println!("Player is going Right!"),
        Direction::Wrong => println!("Wrong direction"),
    }
}

use crate::garden::vegetables::Asparagus;

pub mod garden; // Compiler includes code it finds in garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
use std::fs::{self, File}   ;
use std::io::{self, Read};
use std::io::ErrorKind;
fn main() {
    // let f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating a file"),
    //         }
    //         other => panic!("Random ass error occured")
    //     }
    // };
    read_username_from_file();

}


// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }


// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
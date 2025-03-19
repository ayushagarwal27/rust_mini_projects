use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_result = File::open("hey.txt").expect("file not found!");

    // let file = match greeting_result {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hey.txt") {
    //             Ok(fc) => fc,
    //             Err(err) => panic!("Error in creating file: {:?}", err),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {:?}", other_error)
    //         }
    //     },
    // };
}

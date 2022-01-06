use std::env;
use std::fmt;
use std::fmt::Display;

struct Student{
    // Rust has no classes, so all class like objects are structs
    first_name: String,
    last_name: String,
    id: u32
}

//Instantiate a struct with impl

impl Student {
    fn new(first: &str, last: &str, id: u32) -> Student {
        Student{
        first_name: String::from(first),
        last_name: String::from(last),
        id
    }
}

}


fn main() {

    // read file from args
    let file_name = env::args().nth(1);
    
    match file_name{
        None => panic!("I need a file name!"),
        Some(f) => {println!("{}", f)}
    }

}

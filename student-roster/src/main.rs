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
    println!("Hello, world!");
}

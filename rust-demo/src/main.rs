// fn is the function declarer

fn main() {
    
    
    
    // conditionals
    // A Vec is like a list in Python 
    //let names: Vec<&str> = vec!["Tag", "Husky", "D0nut", "GoPro"]
    
    // var keyword is 'let' like in Nim   
    // let is immutable, but there is no "mutable" variable like var
    // if you want to have a changable var, you need to give it the mut keyword
    // you never really work with a straight str, you usually work with the mem reference (thus the &str)
    // there is also String type
    let name: &str = "Husky";
    
    if name == "Husky" {
        println!("Oh hai guy");
    } else if name == "Taggart" {
        println!("Hey Tag :)")
    } else {
        println!("Hello, {}", name);
    // println is actually a macro (meta programming)
    // builds new code to perform a given function
    }
}


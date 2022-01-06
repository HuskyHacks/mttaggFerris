// fn is the function declarer

fn main() {
    // var keyword is 'let' like in Nim   
    // let is immutable, but there is no "mutable" variable like var
    // if you want to have a changable var, you need to give it the mut keyword
    // you never really work with a straight str, you usually work with the mem reference (thus the &str)
    // there is also String type
    let name: &str = "Husky";
    
    println!("Hello, {}", name);
    // println is actually a macro (meta programming)
    // builds new code to perform a given function
}


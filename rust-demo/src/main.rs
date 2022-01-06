// fn is the function declarer
fn exclaim(n: &str) -> String{
    let mut res = String::from(n);
    res.push_str("!");
    
    // if you omit the return statement, it is implied
    // i.e. if you just write 'res', the same thing happens
    return res;
}

fn main() {
    // A Vec is like a list in Python 
    let names: Vec<&str> = vec!["Tag", "Husky", "D0nut", "GoPro"];
    let exclaimed: Vec<String> = names
    .into_iter()
    .map(exclaim)
    .collect();
    // var keyword is 'let' like in Nim   
    // let is immutable, but there is no "mutable" variable like var
    // if you want to have a changable var, you need to give it the mut keyword
    // you never really work with a straight str, you usually work with the mem reference (thus the &str)
    // there is also String type
    //let name: &str = "Husky";


    // closures
    let num: Vec<u32> = (1..10).collect();

    let squares: Vec<u32> = num
    .into_iter()
    // map and return a new value
    .map(|n:u32| n * n)
    .collect();

    for s in squares {
        println!("{}", s)
    }


    // for loop in rust!
    for n in exclaimed {
        // conditionals!
        //if n == "Husky" {
        //    println!("Oh hai guy");
        //} else if n == "Taggart" {
         //   println!("Hey Tag :)")
        //} else {
            println!("Hello, {}", n);
        // println is actually a macro (meta programming)
        // builds new code to perform a given function
        }
    }


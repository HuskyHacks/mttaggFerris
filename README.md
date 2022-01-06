# mttaggFerris
Taggart's Intro to Rust Stream

Interesting tidbits:
- Rust binaries are (usually) statically compiled, so they're thicc.
- "///" are used to create docstrings/documentation

Site: https://rust-lang.org

## Why Should I Care About Rust?
Speed? Speed is cool and all but the way Rust really shines is its ability to allow for collaboration and express ideas in code.

Rust isn't the first lang to implement memory safety as a core design feature but might be one of the best examples of it.

## Installing Rust

Site: https://rustup.rs/

On Linux:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Reload PATH in current shell:
```
$ source $HOME/.cargo/env
```
## Updating
```
$ rustup update stable
```

## Cargo
Cargo is the package manager, project structure maintainer, package installer, etc

"Your one stop shop for basically every task you want" -d0nutptr

New package:
```
$ cargo new rust-demo
```

### Cargo.toml
Dependency list. Need to add dependencies manually but will auto update.

### Cargo run
```
$ cargo run
```
... from the working directory runs the code from the entrypoint.

## VS Code Rust Analyzer plugin
Download rust-analyzer. It's number two if you search plugins for rust. It's better than the first hit.



## Rust Four Fundamental Structures
### Variables

Vars are assigned with `let`, like in Nim. Unlike Nim, there is no mutable `var`. Instead, you just add the `mut` keyword to any variable you want
```rust
 // var keyword is 'let' like in Nim   
    // let is immutable, but there is no "mutable" variable like var
    // if you want to have a changable var, you need to give it the mut keyword
    // you never really work with a straight str, you usually work with the mem reference (thus the &str)
    // there is also String type
    let name: &str = "Husky";
    
    // if we wanted mutable:
    // let name mut: &str = "Husky";

    println!("Hello, {}", name);
```

### Conditionals
Pretty easy:
```rust
 if n == "Husky" {
            println!("Oh hai guy");
        } else if n == "Taggart" {
            println!("Hey Tag :)")
        } else {
            println!("Hello, {}", n);
```

### Loops
Also pretty easy:
```rust
  // for loop in rust!
    for n in names {
        // conditionals!
        if n == "Husky" {
            println!("Oh hai guy");
        } else if n == "Taggart" {
            println!("Hey Tag :)")
        } else {
            println!("Hello, {}", n);

```

### Functions
```rust
// fn is the function declarer
fn exclaim(n: &str) -> String{
    let mut res = String::from(n);
    res.push_str("!");
    
    // if you omit the return statement, it is implied
    // i.e. if you just write 'res', the same thing happens
    return res;
}
```

## Rust Macros
Macros operate on the text of the code. 

WHen you give a macro a string, it works the abstract syntax tree to build Rust code based on what you provided.

d0nutptr: "when you write vec![1, (2 + 2), 3, 4] rust doesn't see text, it literally sees expressions in the abstract syntax tree and can apply syntax rules that only apply to expressions"

## Crates.io
Package repo for rust.
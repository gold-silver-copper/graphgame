use english::*;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let b = English::noun("boop", &Number::Plural);
    println!("Hello, world! {}", b);
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();
    }
}

use english::*;
use graphgame::*;
use std::io::{self, Write};

fn main() {
    let b = English::noun("boop", &Number::Plural);
    println!("Hello, world! {}", b);
    let mut lg = LocationGraph::new();
    lg.add_location(LocationID(1), "Village", "A small peaceful village.");
    lg.add_location(LocationID(2), "Forest", "A dark and mysterious forest.");
    lg.add_connection(
        LocationID(1),
        LocationID(2),
        PathType::Road,
        TravelDirection::North,
    );
    lg.add_connection(
        LocationID(2),
        LocationID(1),
        PathType::Road,
        TravelDirection::South,
    );

    let mut current_location = LocationID(1);
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();
    }
}

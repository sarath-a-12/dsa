mod algorithms;

use std::env;
use algorithms::{algorithm_a};

fn main() {
    let args: Vec::<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <algorithm>");
        return;
    }

    match args[1].as_str() {
        "A" => algorithm_a::run(),
        _  => println!("Unknown algorithm!"),
    }
}

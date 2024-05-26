use core::panic;
use std::{env, fs};

fn main() {
    // NOTE: if the program needs support invalid unicode use std::env::args_os
    // NOTE: args() returns an iterator and collect turn the iterator into a Vector in this case
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("query: {}", config.query);
    println!("File: {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // NOTE: using clone makes a copy of the full data we are using inside Config.
    // This takes more time and memory than storing a reference, but making it like this makes the
    // code more straightforward and remove the necessity of handling lifetimes.
    // In this particular case, giving up a little performance is worth it
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!(
                "You should provide at least 2 arguments. \nExample: cargo run -- query file_path"
            );
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

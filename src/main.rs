use std::{env, fs};

fn main() {
    // NOTE: if the program needs support invalid unicode use std::env::args_os
    // NOTE: args() returns an iterator and collect turn the iterator into a Vector in this case
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

use std::env;

fn main() {
    // NOTE: if the program needs support invalid unicode use std::env::args_os
    // NOTE: args() returns an iterator and collect turn the iterator into a Vector in this case
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);
}

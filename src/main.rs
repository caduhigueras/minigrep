use minigrep::Config;
use std::{env, process};

fn main() {
    // NOTE: if the program needs support invalid unicode use std::env::args_os
    // NOTE: args() returns an iterator and collect turn the iterator into a Vector in this case
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parasing arguments: {err}");
        process::exit(1);
    });

    // NOTE: not using unwrap_or_else in this case since OK() does not return a value we want to
    // unwrap (Like config returns Config). Here we only need to handle when an error occurs.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

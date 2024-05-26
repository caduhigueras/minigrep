use std::{env, error::Error, fs, process};

fn main() {
    // NOTE: if the program needs support invalid unicode use std::env::args_os
    // NOTE: args() returns an iterator and collect turn the iterator into a Vector in this case
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parasing arguments: {err}");
        process::exit(1);
    });

    println!("query: {}", config.query);
    println!("File: {}", config.file_path);

    // NOTE: not using unwrap_or_else in this case since OK() does not return a value we want to
    // unwrap (Like config returns Config). Here we only need to handle when an error occurs.
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// NOTE: Box<dyn Error> ia trait object that implements the Error trait. Dyn - dynamic
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // NOTE: on an error, ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // NOTE: using clone makes a copy of the full data we are using inside Config.
    // This takes more time and memory than storing a reference, but making it like this makes the
    // code more straightforward and remove the necessity of handling lifetimes.
    // In this particular case, giving up a little performance is worth it
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(
                "You should provide at least 2 arguments. \nExample: cargo run -- query file_path",
            );
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

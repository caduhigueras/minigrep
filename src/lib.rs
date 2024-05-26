use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // NOTE: using clone makes a copy of the full data we are using inside Config.
    // This takes more time and memory than storing a reference, but making it like this makes the
    // code more straightforward and remove the necessity of handling lifetimes.
    // In this particular case, giving up a little performance is worth it
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

// NOTE: Box<dyn Error> ia trait object that implements the Error trait. Dyn - dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // NOTE: on an error, ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}' in {}", config.query, config.file_name);

    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}

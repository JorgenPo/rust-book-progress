use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}' in {}", config.query, config.file_name);

    if let Err(error) = minigrep::run(config) {
        println!("Application error: {}", error);
        process::exit(1);
    }
}

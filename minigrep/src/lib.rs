use std::error::Error;
use std::{fs, env};

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            None => return Err("Didn't get a query string"),
            Some(query) => query,
        };

        let file_name = match args.next() {
            None => return Err("Didn't get a filename string"),
            Some(file) => file,
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{
            query,
            file_name,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/// Search for query in contents and return vector of lines
/// that contain a query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_right_arguments() {
        let args: Vec<String> = vec![
            String::from("/bin/minigrep"), String::from("text"), String::from("filename.txt")];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.file_name, args[2]);
        assert_eq!(config.query, args[1]);
    }

    #[test]
    #[should_panic]
    fn test_new_with_wrong_arguments() {
        let args: Vec<String> = vec![String::from("/bin/minigrep"), String::from("text")];

        let _ = Config::new(&args).unwrap();
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
extern crate regex;

use std::io::{stdin, Read, Error};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

enum Command {
    Add {
        name: String,
        department: String
    },
    Stop,
    List, // List all added employees
    Help,
}

/// Try to parse a command from the input string
fn parse_command(input: &str) -> Option<Command> {
    let pattern = Regex::new(r"Add (.*) to (.*)\.?").expect("Failed to compile a regex");

    for command in pattern.captures_iter(input) {
        return Some(Command::Add{
            name: String::from(&command[1]),
            department: String::from(&command[2])
        });
    }

    None
}

struct EmployeesDatabase(HashMap<String, String>);

impl EmployeesDatabase {
    fn new() -> Self {
        Self{ 0: HashMap::new() }
    }

    fn insert(&mut self, name: String, department: String) -> Option<String> {
        self.0.insert(name.to_lowercase(), department.to_lowercase())
    }
}

impl fmt::Display for EmployeesDatabase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (employee, department) in &self.0 {
            f.write_fmt(format_args!("{} works in {}\n", employee, department))?
        }

        Ok(())
    }
}

/// Try to read from stdin and parse a command
fn get_command() -> Option<Command> {
    let mut input = String::new();
    if let Err(err) = stdin().read_line(&mut input) {
        println!("Failed to read a line from the user: {}", err);
        return None;
    }

    let input = input.trim();

    if input.is_empty() {
        return None;
    } else if input.eq_ignore_ascii_case("Stop") || input.eq_ignore_ascii_case("Exit") {
        return Some(Command::Stop);
    } else if input.eq_ignore_ascii_case("List") {
        return Some(Command::List);
    } else if input.eq_ignore_ascii_case("Help") {
        return Some(Command::Help);
    }

    parse_command(&input)
}

fn print_help() {
    println!("Special command list: ");
    println!(" List - Print the list of all employees and their departments");
    println!(" Help - Print this message");
    println!(" Stop|Exit - Terminate the program");
}

fn main() {
    println!("Employee database");
    print_help();
    println!();

    let mut employees = EmployeesDatabase::new();

    loop {
        println!("Enter command matching the pattern 'Add [Person] to [Department]:");

        if let Some(command) = get_command() {
            match command {
                Command::Add {name, department} => {
                    println!("Okay. Adding {} to {} department", name, department);
                    employees.insert(name, department);
                }
                Command::Stop => {
                    println!("Okay. Thanks for using the program!");
                    break;
                },
                Command::List => {
                    println!("Employees: {}", employees);
                },
                Command::Help => {
                    print_help();
                }
            }
        } else {
            println!("Sorry, wrong command, try again");
            continue;
        }
    }
}
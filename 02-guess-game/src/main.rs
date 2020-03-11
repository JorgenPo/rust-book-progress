extern crate rand;

/// Guess game example
/// Chapter 2

use std::io;
use rand::Rng;
use std::cmp::Ordering;

enum ContinueAnswer {
    Yes,
    No
}

fn check_for_continue() -> ContinueAnswer {
    println!("Want to play again?");
    let mut answer = String::new();

    io::stdin().read_line(&mut answer).expect("Failed to read answer!");

    if answer == String::from("Yes") ||
        answer == String::from("yes") ||
        answer == String::from("y") {
        return ContinueAnswer::Yes;
    }

    ContinueAnswer::No
}

fn main() {
    println!("Guess a number game");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please, input your number: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read a guess");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Yes! You win!");

                match check_for_continue() {
                    ContinueAnswer::Yes => {
                       continue
                    },
                    ContinueAnswer::No => {
                        println!("Okay, goodbye!");
                        break;
                    },
                }
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
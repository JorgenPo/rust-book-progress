use std::collections::HashMap;

fn get_mean(vector: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for element in vector.iter() {
        result += *element;
    }

    result / vector.len() as i32
}

fn get_median(vector: &Vec<i32>) -> i32 {
    let mut sorted = vector.clone();
    sorted.sort();

    sorted[vector.len() / 2]
}

fn get_mode(vector: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for element in vector.iter() {
        let elem = map.entry(element).or_insert(0);
        *elem += 1;
    }

    let mut max_rate: i32 = 0;
    let mut mode_number: i32 = 0;
    for (number, rate) in &map {
        if *rate > max_rate {
            max_rate = *rate;
            mode_number = **number;
        }
    }

    mode_number
}

const VOWELS: &str = "aeiou";

fn is_vowel(character: char) -> bool {
    VOWELS.contains(character)
}

fn to_pig_latin(string: &str) -> String {
    if string.is_empty() {
        return String::from(string)
    }

    let mut result = string;

    let mut first_char = result.chars().next().expect("Wrong string");
    first_char = first_char.to_lowercase().next().expect("Wrong char");

    if !is_vowel(first_char) {
        result = &result[first_char.len_utf8()..];
    } else { // Change char to 'h'
        first_char = 'h';
    }

    String::from(format!("{}-{}ay", result, first_char))
}

fn main() {
    println!("Collections chapter exercises");

    let numbers = vec![10, 2, -3, 14, 0, 27, 18, 5, 9, -15, -10, 10, 3, 5, -8, 0, 5];

    println!("Elements: {:?}", numbers);
    println!("Mean is {}", get_mean(&numbers));
    println!("Median is {}", get_median(&numbers));
    println!("Mode is {}", get_mode(&numbers));

    let string1 = "Rust";
    let string2 = "Angel";
    let string3 = "Дед-Мороз";

    println!("'{}' to pig is '{}'", string1, to_pig_latin(string1));
    println!("'{}' to pig is '{}'", string2, to_pig_latin(string2));
    println!("'{}' to pig is '{}'", string3, to_pig_latin(string3));
}
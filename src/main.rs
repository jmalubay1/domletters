/// Jordan Malubay CS461
/// Dominant letters binary
use std::collections::HashMap;
use std::env;
use std::fs;
use text_colorizer::*;

/// Grabs the filename from the first argument.
/// Errors with invalid number of arguments.
fn getname() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!(
            "{} Wrong number of arguments; expected 1, got {}",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    args[0].clone()
}

/// Checks if the word contains only alpha chars.
/// Iterates through the word storing the count of each letter
/// in a hashmap. All chars are stored and checked in lowercase.
/// Return the count of the most frequent char.
fn domcount(word: &str) -> u32 {
    let mut count: u32 = 0;
    if word.chars().all(char::is_alphabetic) {
        let mut map: HashMap<char, u32> = HashMap::new();
        for ch in word.chars() {
            // Letter exists in Hashmap
            if let Some(x) = map.get_mut(&ch.to_ascii_lowercase()) {
                *x += 1;
            // Insert new letter
            } else {
                map.insert(ch.to_ascii_lowercase(), 1);
            }
        }
        // Find the highest letter occurance
        for val in map.values() {
            if val > &count {
                count = *val
            }
        }
    }
    count
}

fn main() {
    let fname = getname();

    // Store entire file as a String.
    // If file does not exist, error and exit.
    let data = match fs::read_to_string(&fname) {
        Ok(v) => v,
        Err(e) => {
            eprint!(
                "{} Filename '{}' does not exist. {:?}",
                "Error:".red().bold(),
                fname,
                e
            );
            std::process::exit(1);
        }
    };

    // Split the string on all whitespace chars
    let words = data.split_whitespace();
    let mut count = 0;
    // Add the dominant count for each word
    for word in words {
        count += domcount(word)
    }
    println!("{}", count)
}

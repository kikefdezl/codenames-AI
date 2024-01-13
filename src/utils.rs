use std::io::{self};
use std::process;

pub fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read guess.");
    let trimmed_input = input.trim();
    if trimmed_input.to_lowercase() == "exit" {
        println!("Exiting.");
        process::exit(0);
    }
    return trimmed_input.to_string();
}

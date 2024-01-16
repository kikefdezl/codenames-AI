use std::io;

use crate::agent::play_agent_game;
use crate::spymaster::play_spymaster_game;

mod agent;
mod ai;
mod data;
mod spymaster;
mod utils;
mod word_board;

const CHOICE_RANGE: (u8, u8) = (1, 3);

fn main() {
    println!("{:=^70}", " Welcome to codenames! ");
    println!("");

    loop {
        println!(
            "Select what you would like to play as [{}-{}]",
            CHOICE_RANGE.0.to_string(),
            CHOICE_RANGE.1.to_string()
        );
        println!("1. Spymaster");
        println!("2. Agent");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice.");

        match choice.trim().parse::<u8>() {
            Ok(1) => play_spymaster_game(),
            Ok(2) => play_agent_game(),
            Ok(3) => break,
            Ok(_) | Err(_) => println!("Invalid input."),
        }
    }
    println!("Exiting.");
}

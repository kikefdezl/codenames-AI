use std::io;

use crate::agent::play_agent_game;
use crate::spymaster::play_spymaster_game;

mod agent;
mod ai;
mod clue;
mod data;
mod spymaster;
mod utils;
mod word_board;

fn game_selection_menu() -> u8 {
    let choice_range = (1, 3);
    loop {
        println!(
            "Select what you would like to play as [{}-{}]",
            choice_range.0.to_string(),
            choice_range.1.to_string()
        );
        println!("1. Spymaster");
        println!("2. Agent");
        println!("3. Exit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice.");
        match choice.trim().parse::<u8>() {
            Ok(num) if choice_range.0 <= num && num <= choice_range.1 => {
                return num;
            }
            Ok(_) => {
                println!("Invalid input.");
                continue;
            }
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        }
    }
}

fn main() {
    println!("{:=^70}", " Welcome to codenames! ");
    println!("");

    loop {
        let gametype = game_selection_menu();
        if gametype == 1 {
            play_spymaster_game();
        }
        if gametype == 2 {
            play_agent_game();
        }
        if gametype == 3 {
            break;
        }
    }
    println!("Exiting.");
}

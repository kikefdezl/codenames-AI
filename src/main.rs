use std::io;

use crate::spymaster::play_spymaster_game;

mod spymaster;


fn game_selection_menu() -> u8 {
    let mut gametype: u8 = 0;
    let choice_range = (1, 2);
    while gametype < choice_range.0 || choice_range.1 < gametype  { 
        println!("Select what you would like to play as [{}-{}]",
                 choice_range.0.to_string(), choice_range.1.to_string());
        println!("1. Spymaster");
        println!("2. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice."); 
        gametype = choice.trim().parse().expect("Please type a number.");
    }
    return gametype;
}


fn main() {
    println!("{:=^70}", "Welcome to codenames!");
    println!("");

    loop {
        let gametype = game_selection_menu();
        if gametype == 1 {
            play_spymaster_game()
        }
        if gametype == 2 {
            break;
        }
    }
    println!("Exiting.");
}

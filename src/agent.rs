use std::io;

use crate::common::{ Board, print_board, get_team_mask, get_word_board };
use crate::constants:: { BOARD_SIZE };

pub fn play_agent_game() {
    let board = Board {
        words: get_word_board(),
        team_mask: get_team_mask(),
        guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE]
    };

    let mut response = String::new();
    loop {
        print_board(&board);
        io::stdin().read_line(&mut response)
            .expect("Failed to read choice."); 
    }
}

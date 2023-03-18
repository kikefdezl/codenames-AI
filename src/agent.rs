use crate::common::{ Board, print_board, get_team_mask, get_word_board, 
    read_user_input, cross_guessed_words };
use crate::constants:: BOARD_SIZE;

fn word_in_board(input_word: &String, word_board: &Vec<Vec<String>>) -> bool {
    for row in word_board {
        for word in row {
            if input_word.to_lowercase() == word.to_lowercase() {
                return true;
            }
        }
    }
    return false;
}

pub fn play_agent_game() {
    let mut board = Board {
        words: get_word_board(),
        team_mask: get_team_mask(),
        guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE]
    };

    let mut guess = String::new();

    // Game loop
    loop {
        print_board(&board, false);

        // AI provides clue here

        while !word_in_board(&guess, &board.words) {
            println!("Provide your guess:");
            guess = read_user_input();
        }
        let guessed_word = vec![guess.to_string(); 1];
        cross_guessed_words(&mut board, &guessed_word);

        // reset
        println!("");
        guess = "".to_string();
    }
}

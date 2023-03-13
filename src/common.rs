use colored::*;

use crate::constants::{ BOARD_SIZE };

pub struct Board {
    pub words: Vec<Vec<String>>,
    pub team_mask: Vec<Vec<bool>>,
    pub guessed_mask: Vec<Vec<bool>>,
}

fn get_max_word_length(board: &Vec<Vec<String>>) -> usize {
    let mut max = 0;
    for row in board {
        for word in row {
            let wordlength = word.len();
            if wordlength > max {
                max = wordlength;
            }
        }
    }
    return max;
}

pub fn print_board(board: &Board) {
    let print_width = get_max_word_length(&board.words) + 2;
    let mut your_words: Vec<String> = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let mut word = board.words[row][col].to_string();
            let mut padding = print_width - word.len(); 

            if board.team_mask[row][col] {
                word = word.red().to_string();
                if !board.guessed_mask[row][col] {
                    your_words.push(board.words[row][col].to_string());
                }
            }
            if board.guessed_mask[row][col] {
                word = "".to_string();
                padding = print_width;
            }
            print!("{:>padding$}", "");
            print!("{}", word.to_string());
           
        }
        println!("");
    }
    let joined = your_words.join(", ");
    println!("Your words: {joined}");
}


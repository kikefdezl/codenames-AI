use colored::*;
use std::fs::File;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::path::Path;
use std::io::{ self, BufRead };

use crate::constants:: { BOARD_SIZE, WORD_LIST_FILE, NUM_WORDS_TO_GUESS }; 

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

pub fn get_team_mask() -> Vec<Vec<bool>> {
    let mut count = 0;
    let mut mask = vec![vec![false; BOARD_SIZE]; BOARD_SIZE];
    loop {
        let col = rand::thread_rng().gen_range(0..BOARD_SIZE);
        let row = rand::thread_rng().gen_range(0..BOARD_SIZE);
        if mask[col][row] == false {
            mask[col][row] = true;
            count = count + 1;
            if count == NUM_WORDS_TO_GUESS {
                break;
            }
        }
    }
    return mask;
}

pub fn get_word_board() ->  Vec<Vec<String>> {
    // Open the file
    let path = Path::new(WORD_LIST_FILE);
    let file = File::open(&path).expect("Failed to open file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            if let Ok(l) = line {
                l
            } else {
                panic!("Failed to read the line from file");
            }
        })
        .collect();

    // Shuffle the vector
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = lines
        .iter()
        .map(|s| s.to_string())
        .collect();
    words.shuffle(&mut rng);

    // Select BOARD_SIZE*BOARD_SIZE words, and split it into a 2D vector
    let selected_words: Vec<String> = words
        .iter()
        .take(BOARD_SIZE*BOARD_SIZE)
        .cloned()
        .collect();
    let board: Vec<Vec<String>> = selected_words
        .chunks(BOARD_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    return board;
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


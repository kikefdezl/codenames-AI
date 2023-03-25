use colored::*;
use pyo3::prelude::*;
use std::process;
use std::fs::File;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::path::Path;
use std::io::{ self, BufRead };

use crate::constants:: { BOARD_SIZE, WORDS_CODENAMES_LIST, NUM_WORDS_TO_GUESS }; 

pub struct Board {
    pub words: Vec<Vec<String>>,
    pub team_mask: Vec<Vec<bool>>,
    pub guessed_mask: Vec<Vec<bool>>,
}

pub fn compute_word_to_words_similarity (
    reference_word: &String, 
    words: &Vec<String>
) -> PyResult<Vec<f32>> {

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ai_tools = PyModule::import(py, "ai_tools")?;
        let result: Vec<f32> = ai_tools
            .getattr("compute_word_to_words_similarity")?
            .call1((reference_word.clone(), words.clone(), ))?
            .extract()?;
        Ok(result)
    })
}

pub fn get_remaining_words(board: &Board) -> Vec<String> {
    let mut remaining_words = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if !board.guessed_mask[row][col] {
                remaining_words.push(board.words[row][col].to_string());
            }
        }
    }
    return remaining_words;
}


pub fn get_remaining_team_words(board: &Board) -> Vec<String> {
    let mut words = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if !board.guessed_mask[row][col] && board.team_mask[row][col] {
                words.push(board.words[row][col].to_string());
            }
        }
    }
    return words;
}


pub fn get_remaining_non_team_words(board: &Board) -> Vec<String> {
    let mut words = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if !board.guessed_mask[row][col] && !board.team_mask[row][col] {
                words.push(board.words[row][col].to_string());
            }
        }
    }
    return words;
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

pub fn read_word_file(path: &str) -> Vec<String> {
    let path = Path::new(&path);
    let file = File::open(&path).expect("Failed to open file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            if let Ok(l) = line {
                l.to_uppercase()
            } else {
                panic!("Failed to read the line from file");
            }
        })
        .collect();
    return lines;
}

pub fn get_word_board() ->  Vec<Vec<String>> {
    // Open the file
    let lines = read_word_file(WORDS_CODENAMES_LIST);
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

pub fn read_user_input() -> String {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Failed to read guess."); 
    let trimmed_input = input.trim();
    if trimmed_input.to_lowercase() == "exit" {
        println!("Exiting.");
        process::exit(0);
        }
    return trimmed_input.to_string(); 
}

pub fn cross_guessed_words(board: &mut Board, max_words: &Vec<String>) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            for word in max_words {
                if board.words[row][col].to_lowercase() == *word.to_lowercase() {
                    board.guessed_mask[row][col] = true;
                }
            }
        }
    }
}

pub fn print_board(board: &Board, colors: bool) {
    let print_width = get_max_word_length(&board.words) + 2;
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let mut word = board.words[row][col].to_string();
            let mut padding = print_width - word.len(); 

            if board.team_mask[row][col] && colors {
                word = word.red().to_string();
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
}

pub fn print_your_words(board: &Board) {
    let mut your_words: Vec<String> = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board.team_mask[row][col] && !board.guessed_mask[row][col] {
                your_words.push(board.words[row][col].to_string());
            }
        }
    }
    let joined = your_words.join(", ");
        println!("Your words: {joined}");
}

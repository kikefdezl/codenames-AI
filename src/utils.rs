use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

use rand::prelude::SliceRandom;

use crate::constants::{BOARD_SIZE, WORDS_CODENAMES_LIST};

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

pub fn get_word_board() -> Vec<Vec<String>> {
    // Open the file
    let lines = read_word_file(WORDS_CODENAMES_LIST);
    // Shuffle the vector
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    words.shuffle(&mut rng);

    // Select BOARD_SIZE*BOARD_SIZE words, and split it into a 2D vector
    let selected_words: Vec<String> = words
        .iter()
        .take(BOARD_SIZE * BOARD_SIZE)
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


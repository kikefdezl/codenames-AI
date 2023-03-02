use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::path::Path;
use std::io::{ self, BufRead };
use colored::*;

const WORD_LIST_FILE: &str = "./data/wordlist-eng.txt";
const BOARD_SIZE: usize = 5;
const NUM_WORDS_TO_GUESS: usize = 9;

fn get_word_board() ->  Vec<Vec<String>> {
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
    let mut words: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    words.shuffle(&mut rng);

    // Select BOARD_SIZE*BOARD_SIZE words, and split it into a 2D vector
    let selected_words: Vec<String> = words.iter().take(BOARD_SIZE*BOARD_SIZE).cloned().collect();
    let board: Vec<Vec<String>> = selected_words.chunks(BOARD_SIZE).map(|chunk| chunk.to_vec()).collect();

    return board;
}


fn get_word_mask() -> Vec<Vec<bool>> {
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


fn print_board(board: &Vec<Vec<String>>, mask: &Vec<Vec<bool>>) {
    let print_width = get_max_word_length(&board) + 2;
    let mut your_words: Vec<String> = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if mask[col][row] {
                let colored_word = board[row][col].red();
                print!("{:>print_width$}", colored_word); 
                your_words.push(board[row][col].to_string());
            }
            else {
                print!("{:>print_width$}", board[row][col]);
            }
        }
        println!("");
    }
    let joined = your_words.join(", ");
    println!("Your words: {joined}");
}


fn main() {
    println!("{:=^70}", "Welcome to codenames!");
    println!("");

    let word_board: Vec<Vec<String>> = get_word_board();
    let word_mask: Vec<Vec<bool>> = get_word_mask();

    print_board(&word_board, &word_mask);
}

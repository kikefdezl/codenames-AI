use rand::seq::SliceRandom;
use std::fs::File;
use std::path::Path;
use std::io::{ self, BufRead };

const WORD_LIST_FILE: &str = "./src/wordlist-eng.txt";
const BOARD_SIZE: usize = 5;

fn get_word_board() ->  Vec<Vec<String>> {
    // Open the file
    let path = Path::new(WORD_LIST_FILE);
    let file = File::open(&path).expect("Failed to open file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
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


fn print_board(board: &Vec<Vec<String>>) {
    let width = get_max_word_length(&board) + 2;

    for row in board {
        for word in row {
            print!("{word:>width$}");
        }
        println!("");
    }
}


fn main() {
    println!("{:=^70}", "Welcome to codenames!");
    println!("");

    let word_board: Vec<Vec<String>> = get_word_board();

    print_board(&word_board);
}

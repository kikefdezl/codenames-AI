use rand::Rng;
use rand::prelude::SliceRandom;
use std::fs::File;
use std::path::Path;
use std::io::{ self, BufRead };
use pyo3::prelude::*;
use regex::Regex;
use colored::*;

const WORD_LIST_FILE: &str = "./data/wordlist-eng.txt";
const BOARD_SIZE: usize = 5;
const NUM_WORDS_TO_GUESS: usize = 9;


struct Board {
    words: Vec<Vec<String>>,
    team_mask: Vec<Vec<bool>>,
    guessed_mask: Vec<Vec<bool>>,
}

fn compute_word_similarity(words: Vec<String>) -> PyResult<Vec<f32>> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ai_tools = PyModule::import(py, "ai_tools")?;
        let result: Vec<f32> = ai_tools
            .getattr("compute_similarity")?
            .call1((words,))?
            .extract()?;
        println!("{:?}", result);
        Ok(result)
    })
}

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


fn get_team_mask() -> Vec<Vec<bool>> {
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


fn print_board(board: &Board) {
    let print_width = get_max_word_length(&board.words) + 2;
    let mut your_words: Vec<String> = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board.team_mask[col][row] {
                let colored_word = board.words[row][col].blue();
                print!("{:>print_width$}", colored_word); 
                your_words.push(board.words[row][col].to_string());
            }
            else {
                print!("{:>print_width$}", board.words[row][col]);
            }
        }
        println!("");
    }
    let joined = your_words.join(", ");
    println!("Your words: {joined}");
}


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


fn get_remaining_words(board: &Board) -> Vec<String> {
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


fn play_spymaster_game() {
    let mut board = Board {
        words: get_word_board(),
        team_mask: get_team_mask(),
        guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE]
    };

    let clue_re = Regex::new(r"\w \d").unwrap();
    let mut choice = String::new();
    print_board(&board);
    while !clue_re.is_match(&choice) {
        println!("Provide a clue:");
        io::stdin().read_line(&mut choice).expect("Failed to read choice."); 
    }
    let remaining_words = get_remaining_words(&board);
    println!("{:?}", remaining_words);
    compute_word_similarity(remaining_words).expect("Couldn't get result from AI model");
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

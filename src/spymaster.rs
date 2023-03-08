use rand::Rng;
use rand::prelude::SliceRandom;
use std::fs::File;
use std::path::Path;
use std::io::{ self, BufRead };
use pyo3::prelude::*;
use colored::*;
use regex::Regex;

const WORD_LIST_FILE: &str = "./data/wordlist-eng.txt";
const BOARD_SIZE: usize = 5;
const NUM_WORDS_TO_GUESS: usize = 9;


struct Board {
    words: Vec<Vec<String>>,
    team_mask: Vec<Vec<bool>>,
    guessed_mask: Vec<Vec<bool>>,
}

fn compute_word_to_words_similarity (
    reference_word: &String, 
    words: &Vec<String>
) -> PyResult<Vec<f32>> {

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ai_tools = PyModule::import(py, "ai_tools")?;
        let result: Vec<f32> = ai_tools
            .getattr("compute_word_to_words_similarity")?
            .call1((reference_word.clone(), words.clone(),))?
            .extract()?;
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

fn get_n_max_words(words: &Vec<String>, 
                   values: &Vec<f32>, 
                   n: usize) -> Vec<String> {
    let mut pairs: Vec<_> = words.iter().zip(values.iter()).collect();
    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let max_words: Vec<String> = pairs
        .iter()
        .take(n)
        .map(|(s, _)| s.to_string())
        .collect();
    return max_words;
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

pub fn play_spymaster_game() {
    let mut board = Board {
        words: get_word_board(),
        team_mask: get_team_mask(),
        guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE]
    };

    let clue_re = Regex::new(r"\w \d").unwrap();
    let mut clue = String::new();
    // Game loop
    loop {
        print_board(&board);

        // Get clue from user
        while !clue_re.is_match(&clue) {
            println!("Provide a clue:");
            io::stdin().read_line(&mut clue).expect("Failed to read choice."); 
        }
        let clue_parts: Vec<&str> = clue.split(' ').collect();
        let reference_word: String = clue_parts[0].to_string();
        let n_words_referenced: usize = clue_parts[clue_parts.len() - 1]
            .trim()
            .parse()
            .expect("Last value of the clue is not an integer");

        let remaining_words = get_remaining_words(&board);
        let result: Vec<f32> = compute_word_to_words_similarity(
            &reference_word, 
            &remaining_words)
            .expect("Couldn't get result from AI model");
        let max_words: Vec<String> = get_n_max_words(
            &remaining_words, &result, n_words_referenced);
        println!("max_words: {:?}", max_words);
        clue = "".to_string();
    }
}

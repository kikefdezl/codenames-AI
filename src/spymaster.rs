use std::process;
use std::io;
use pyo3::prelude::*;
use regex::Regex;

use crate::common::{ Board, print_board, get_team_mask, get_word_board};
use crate::constants:: { BOARD_SIZE }; 


fn compute_word_to_words_similarity (
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

fn get_remaining_team_words(board: &Board) -> i8 {
    let mut n: i8 = 0;
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if !board.guessed_mask[row][col] && board.team_mask[row][col] {
                n += 1;
            }
        }
    }
    return n;
}
fn cross_guessed_words(board: &mut Board, max_words: &Vec<String>) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            for word in max_words {
                if board.words[row][col] == *word {
                    board.guessed_mask[row][col] = true;
                }
            }
        }
    }
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
            clue = "".to_string();
            println!("Provide a clue:");
            io::stdin().read_line(&mut clue).expect("Failed to read choice."); 
            if clue.trim() == "exit" {
                println!("Exiting.");
                process::exit(0);
            }
        }
        let clue_parts: Vec<&str> = clue.trim().split(' ').collect();
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
        println!("AI guesses: {:?}", max_words);
        cross_guessed_words(&mut board, &max_words);
        if get_remaining_team_words(&board) == 0 {
            break;
        }
        clue = "".to_string();
        println!(" ");
    }
    println!("You win!");
}

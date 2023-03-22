use std::io::{stdout, Write};
use crate::constants:: { BOARD_SIZE, WORDS_10K_LIST };
use crate::common::{ 
    Board, 
    print_board, 
    get_team_mask, 
    get_word_board, 
    get_remaining_team_words,
    get_remaining_non_team_words,
    read_user_input, 
    cross_guessed_words, 
    read_word_file,
    compute_word_to_words_similarity
};


fn find_max_value(numbers: &Vec<f32>) -> Option<f32> {
    numbers.iter().fold(None, |max, current| {
        match max {
            Some(value) => Some(value.max(*current)),
            None => Some(*current),
        }
    })
}


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

fn give_clue(board: &Board, words_10k: &Vec<String>) -> String {
    let team_words = get_remaining_team_words(board);
    let non_team_words = get_remaining_non_team_words(board);
    let mut max_count: usize = 0;
    let mut best_clue = String::new();

    print!("Thinking...");
    stdout().flush().unwrap();
    for word in words_10k {
        let team_words_results = compute_word_to_words_similarity(word, &team_words)
            .expect("Couldn't get result from AI model");
        let non_team_words_results = compute_word_to_words_similarity(word, &non_team_words)
            .expect("Couldn't get result from AI model");
        let threshold = find_max_value(&non_team_words_results).expect("Vector is empty!"); 
        let words_above_threshold = team_words_results
            .iter()
            .filter(|&x| *x > threshold)
            .count();
        if words_above_threshold > max_count {
            max_count = words_above_threshold;
            best_clue = word.to_string();
        }
    }
    println!("\r                         ");
    return format!("{} {}", best_clue.to_uppercase(), max_count);
}



pub fn play_agent_game() {
    let mut board = Board {
        words: get_word_board(),
        team_mask: get_team_mask(),
        guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE]
    };
    let words_10k = read_word_file(WORDS_10K_LIST);
    let mut guess = String::new();

    // Game loop
    loop {
        print_board(&board, false);

        // AI provides clue here
        let clue = give_clue(&board, &words_10k);
        println!("Clue: {clue}");

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

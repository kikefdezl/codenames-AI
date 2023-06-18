use crate::common::{
    compute_words_to_words_similarity, cross_guessed_words, get_remaining_non_team_words,
    get_remaining_team_words, get_team_mask, get_word_board, print_board, read_user_input,
    read_word_file, Board,
};
use crate::constants::{BOARD_SIZE, RISK_THRESHOLD, WORDS_COMMON_LIST};
use std::io::Write;


fn find_max_value(numbers: &Vec<f32>) -> Option<f32> {
    numbers.iter().fold(None, |max, current| match max {
        Some(value) => Some(value.max(*current)),
        None => Some(*current),
    })
}

fn word_in_board(input_word: &String, board: &Board) -> bool {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if input_word.to_lowercase() == board.words[row][col].to_lowercase()
                && !board.guessed_mask[row][col]
            {
                return true;
            }
        }
    }
    return false;
}

fn give_clue(
    board: &Board,
    words_common: &Vec<String>,
    used_clues: &Vec<String>,
) -> (String, usize) {
    let team_words = get_remaining_team_words(board);
    let non_team_words = get_remaining_non_team_words(board);
    let mut max_count: usize = 0;
    let mut best_clue = String::new();

    print!("Thinking...");
    std::io::stdout().flush().unwrap();

    let words_common_filtered: Vec<String> = words_common
        .iter()
        .filter(|s| !used_clues.contains(&s) && !word_in_board(&s, board))
        .cloned()
        .collect();

    let team_words_results = compute_words_to_words_similarity(&words_common_filtered, &team_words)
        .expect("Couldn't get result from AI model");
    let non_team_words_results = compute_words_to_words_similarity(&words_common_filtered, &non_team_words)
        .expect("Couldn't get result from AI model");

    for (index, word) in words_common_filtered.iter().enumerate() {
        let threshold =
            find_max_value(&non_team_words_results[index]).expect("Vector is empty!") + RISK_THRESHOLD;
        let words_above_threshold = team_words_results[index]
            .iter()
            .filter(|&x| *x > threshold)
            .count();
        if words_above_threshold > max_count {
            max_count = words_above_threshold;
            best_clue = word.to_string();
        }
    }
    print!("\r");
    println!("Done!             ");
    return (best_clue.to_uppercase(), max_count);
}

pub fn play_agent_game() {
    let mut board = Board {
        words: get_word_board(),
        team_mask: get_team_mask(),
        guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
    };
    let words_common = read_word_file(WORDS_COMMON_LIST);
    let mut guess = String::new();
    let mut used_clues: Vec<String> = vec![];
    let mut remaining_team_words = get_remaining_team_words(&board);

    // Game loop
    while remaining_team_words.len() > 0 {
        print_board(&board, false);
        remaining_team_words = get_remaining_team_words(&board);
        println!("{} words remaining.", remaining_team_words.len());

        let (clue, to_guess_count) = give_clue(&board, &words_common, &used_clues);
        used_clues.push(clue.clone());

        let mut guessed_count = 0;
        while guessed_count < to_guess_count {
            println!("Clue: {} {}", clue, to_guess_count - guessed_count);
            while !word_in_board(&guess, &board) {
                println!("Provide your guess:");
                guess = read_user_input();
            }

            if remaining_team_words.contains(&guess.to_uppercase()) {
                println!("Correct!");
                guessed_count += 1;
            } else {
                println!("Wrong...");
                guessed_count = to_guess_count;
            }
            let guessed_word = vec![guess.to_string(); 1];
            cross_guessed_words(&mut board, &guessed_word);

            if get_remaining_non_team_words(&board).len() == 0 {
                println!("You lose!");
                return;
            }

            // reset
            println!("");
            guess = "".to_string();
        }
    }
    println!("You win!");
}

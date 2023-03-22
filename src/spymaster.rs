use regex::Regex;

use crate::constants:: BOARD_SIZE; 
use crate::common::{ 
    Board, 
    print_board, 
    print_your_words,
    get_team_mask, 
    get_word_board, 
    get_remaining_words,
    get_remaining_team_words,
    read_user_input, 
    cross_guessed_words,
    compute_word_to_words_similarity
};


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
        print_board(&board, true);
        print_your_words(&board);

        // Get clue from user
        while !clue_re.is_match(&clue) {
            println!("Provide a clue:");
                clue = read_user_input();
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
        if get_remaining_team_words(&board).len() == 0 {
            break;
        }

        // reset
        clue = "".to_string();
        println!(" ");
    }
    println!("You win!");
}

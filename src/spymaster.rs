use regex::Regex;

use crate::common::{
    Board, compute_words_to_words_similarity, cross_guessed_words,
    get_remaining_team_words, get_remaining_words, get_team_mask, get_word_board, print_board,
    print_your_words, read_user_input,
};
use crate::constants::BOARD_SIZE;

fn get_n_max_words(words: &Vec<String>, values: &Vec<Vec<f32>>, n: usize) -> Vec<String> {
    let mut pairs: Vec<_> = words.iter().zip(values[0].iter()).collect();
    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let max_words: Vec<String> = pairs.iter().take(n).map(|(s, _)| s.to_string()).collect();
    return max_words;
}

pub fn play_spymaster_game() {
    let mut board = Board {
        words: get_word_board(),
        team_mask: get_team_mask(),
        guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
    };

    // Game loop
    loop {
        print_board(&board, true);
        print_your_words(&board);
        let words_on_board = get_remaining_words(&board);
        let clue_re = Regex::new(r"\w \d").unwrap();
        let mut clue_word: String = String::new();
        let mut clue_number = 0;
        let mut clue_ok = false;

        let remaining_words = get_remaining_words(&board);

        // Get clue from user
        while !clue_ok {
            println!("Provide a clue:");
            let clue = read_user_input();

            if !clue_re.is_match(&clue) {
                println!("Invalid format. Format should be: WORD NUM \n");
                continue;
            }

            let clue_parts: Vec<&str> = clue.trim().split(' ').collect();
            clue_word = clue_parts[0].to_string();
            clue_number = clue_parts[1]
                .trim()
                .parse()
                .expect("Last value of the clue is not an integer");

            if words_on_board.contains(&clue_word) {
                println!("You can't use a word that's on the board! \n");
                continue;
            }

            let n_team_words = get_remaining_team_words(&board).len();
            if clue_number > n_team_words {
                println!(
                    "The clue number can't be higher than the number of remaining team words ({n_team_words}). \n");
                continue;
            }
            clue_ok = true;
        }

        let mut clue_word_vec: Vec<String> = Vec::new();
        clue_word_vec.push(clue_word);
        let result: Vec<Vec<f32>> = compute_words_to_words_similarity(&clue_word_vec, &remaining_words)
            .expect("Couldn't get result from AI model");
        let max_words: Vec<String> = get_n_max_words(&remaining_words, &result, clue_number);
        println!("AI guesses: {:?}", max_words);
        cross_guessed_words(&mut board, &max_words);
        if get_remaining_team_words(&board).len() == 0 {
            break;
        }
        println!(" ");
    }
    println!("You win!");
}

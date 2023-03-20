use crate::constants:: { BOARD_SIZE, WORDS_10K_LIST };
use crate::common::{ 
    Board, 
    print_board, 
    get_team_mask, 
    get_word_board, 
    get_remaining_words,
    read_user_input, 
    cross_guessed_words, 
    read_word_file,
    compute_word_to_words_similarity
};

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
    let words_on_board= get_remaining_words(board);
    for word in words_10k {
        println!("{word}");
        let result = compute_word_to_words_similarity(word, &words_on_board);
    }
    return "testing".to_string();
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

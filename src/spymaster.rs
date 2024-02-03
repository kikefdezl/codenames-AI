use lazy_static::lazy_static;
use regex::Regex;

use crate::prompting::prompt_ai_agent;
use crate::utils::{Clue, read_user_input};
use crate::word_board::WordBoard;

lazy_static! {
    static ref CLUE_RE: Regex = Regex::new(r"\w \d").unwrap();
}

pub fn play_spymaster_game() {
    let mut board = WordBoard::new();

    loop {
        board.print(true);
        board.print_your_words();

        // Get clue from user
        let clue: Clue = get_clue_from_user(&board);

        let guesses: Vec<String> = prompt_ai_agent(board.remaining_words(), &clue);
        println!("AI guesses: {:?}", guesses);

        board.cross_guessed_words(&guesses);
        if board.remaining_team_words().len() == 0 {
            break;
        }
        println!(" ");
    }
    println!("You win!");
}

fn get_clue_from_user(board: &WordBoard) -> Clue {
    loop {
        println!("Provide a clue:");
        let clue = read_user_input();

        // Check that the format of the clue is 'WORD NUM'
        if !CLUE_RE.is_match(&clue) {
            println!("Invalid format. Format should be: WORD NUM \n");
            continue;
        }

        let clue_parts: Vec<&str> = clue.trim().split(' ').collect();
        let clue_word = clue_parts[0].to_string();

        let clue_number = clue_parts[1]
            .trim()
            .parse()
            .expect("Last value of the clue is not an integer");

        let words_on_board = board.remaining_words();

        if words_on_board.contains(&clue_word) {
            println!("You can't use a word that's on the board! \n");
            continue;
        }

        let n_team_words = board.remaining_team_words().len();
        if clue_number > n_team_words {
            println!(
                "The clue number can't be higher than the number of remaining team \
                     words ({n_team_words}). \n"
            );
            continue;
        }
        return Clue {
            word: clue_word,
            number: clue_number,
        };
    }
}

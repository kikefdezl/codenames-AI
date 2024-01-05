use crate::ai::prompt_ai_spymaster;
use crate::utils::read_user_input;
use crate::word_board::WordBoard;

pub fn play_agent_game() {
    let mut board = WordBoard::new();
    let mut guess = String::new();
    let mut used_clues: Vec<String> = vec![];

    loop {
        board.print(false);

        let remaining_team_words = board.remaining_team_words();
        println!("{} words remaining.", remaining_team_words.len());

        let clue = prompt_ai_spymaster(board.remaining_team_words(), board.remaining_non_team_words());
        used_clues.push(clue.word.clone());

        let mut remaining_guesses = clue.number;
        while remaining_guesses > 0 {
            println!("Clue: {} {}", clue.word, remaining_guesses);

            while !board.contains(&guess) {
                println!("Provide your guess:");
                guess = read_user_input();
            }

            if remaining_team_words.contains(&guess.to_uppercase()) {
                println!("Correct!");
                remaining_guesses -= 1;
            } else {
                println!("Wrong...");
                remaining_guesses = 0;
            }

            board.cross_guessed_word(&guess);

            if board.remaining_team_words().len() == 0 {
                println!("You win!");
                return;
            }
            if board.remaining_non_team_words().len() == 0 {
                println!("You lose!");
                return;
            }

            // reset
            println!("");
            guess = "".to_string();
        }
    }
}

use crate::clue::Clue;

pub fn prompt_ai_agent(words_on_board: Vec<String>, clue: &Clue) -> Vec<String> {
    words_on_board.into_iter().take(clue.number).collect()
}

pub fn prompt_ai_spymaster(team_words: Vec<String>, non_team_words: Vec<String>) -> Clue {
    Clue {
        word: String::from("hello"),
        number: 1,
    }
}
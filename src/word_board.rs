use colored::Colorize;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::constants::{BOARD_SIZE, NUM_WORDS_TO_GUESS, WORDS_CODENAMES_LIST};
use crate::utils::read_word_file;

pub struct WordBoard {
    pub words: Vec<Vec<String>>,
    pub team_mask: Vec<Vec<bool>>,
    pub guessed_mask: Vec<Vec<bool>>,
}

impl WordBoard {
    pub fn new() -> WordBoard {
        WordBoard {
            words: WordBoard::generate_word_board(),
            team_mask: WordBoard::generate_team_mask(),
            guessed_mask: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn generate_word_board() -> Vec<Vec<String>> {
        // Open the file
        let lines = read_word_file(WORDS_CODENAMES_LIST);
        // Shuffle the vector
        let mut rng = rand::thread_rng();
        let mut words: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        words.shuffle(&mut rng);

        // Select BOARD_SIZE*BOARD_SIZE words, and split it into a 2D vector
        let selected_words: Vec<String> = words
            .iter()
            .take(BOARD_SIZE * BOARD_SIZE)
            .cloned()
            .collect();
        let board: Vec<Vec<String>> = selected_words
            .chunks(BOARD_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect();

        return board;
    }


    fn generate_team_mask() -> Vec<Vec<bool>> {
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


    pub fn remaining_words(&self) -> Vec<String> {
        let mut remaining_words = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if !self.guessed_mask[row][col] {
                    remaining_words.push(self.words[row][col].to_string());
                }
            }
        }
        return remaining_words;
    }

    pub fn remaining_team_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if !self.guessed_mask[row][col] && self.team_mask[row][col] {
                    words.push(self.words[row][col].to_string());
                }
            }
        }
        return words;
    }


    pub fn remaining_non_team_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if !self.guessed_mask[row][col] && !self.team_mask[row][col] {
                    words.push(self.words[row][col].to_string());
                }
            }
        }
        return words;
    }

    fn max_word_length(&self) -> usize {
        let mut max = 0;
        for row in &self.words {
            for word in row {
                let wordlength = word.len();
                if wordlength > max {
                    max = wordlength;
                }
            }
        }
        return max;
    }

    pub fn cross_guessed_word(&mut self, word_to_cross: &str) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.words[row][col].to_lowercase() == word_to_cross.to_lowercase() {
                    self.guessed_mask[row][col] = true;
                }
            }
        }
    }
    pub fn cross_guessed_words(&mut self, words_to_cross: &Vec<String>) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                for word in words_to_cross {
                    if self.words[row][col].to_lowercase() == *word.to_lowercase() {
                        self.guessed_mask[row][col] = true;
                    }
                }
            }
        }
    }

    pub fn print(&self, colors: bool) {
        let print_width = self.max_word_length() + 2;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let mut word = self.words[row][col].to_string();
                let mut padding = print_width - word.len();

                if self.team_mask[row][col] && colors {
                    word = word.red().to_string();
                }

                if self.guessed_mask[row][col] {
                    word = "".to_string();
                    padding = print_width;
                }
                print!("{:>padding$}", "");
                print!("{}", word);
            }
            println!("");
        }
    }

    pub fn print_your_words(&self) {
        let mut your_words: Vec<String> = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.team_mask[row][col] && !self.guessed_mask[row][col] {
                    your_words.push(self.words[row][col].to_string());
                }
            }
        }
        let joined = your_words.join(", ");
        println!("Your words: {joined}");
    }

    pub fn contains(&self, word: &str) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if word.to_lowercase() == self.words[row][col].to_lowercase()
                    && !self.guessed_mask[row][col]
                {
                    return true;
                }
            }
        }
        return false;
    }
}
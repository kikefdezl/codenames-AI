use std::path::PathBuf;

use candle_transformers::models::mistral::Config;
use candle_transformers::models::quantized_mistral::Model as QMistral;
use hf_hub::api::sync::Api;

use crate::ai::mistral::prompt_mistral;
use crate::utils::Clue;

const MODEL_ID: &str = "lmz/candle-mistral";
const MODEL_FILENAME: &str = "model-q4k.gguf";

struct ModelFiles {
    gguf: PathBuf,
    tokenizer: PathBuf,
}

fn download_model() -> ModelFiles {
    let api = Api::new().unwrap();
    let model_filename: PathBuf = api.model(MODEL_ID.to_string()).get(MODEL_FILENAME).unwrap();
    let tokenizer_filename: PathBuf = api
        .model(MODEL_ID.to_string())
        .get("tokenizer.json")
        .unwrap();
    ModelFiles {
        gguf: model_filename,
        tokenizer: tokenizer_filename,
    }
}

fn load_model(model_files: &ModelFiles) -> QMistral {
    let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(&model_files.gguf)
        .expect("Failed to load GGUF file");
    let config = Config::config_7b_v0_1(true);
    QMistral::new(&config, vb).expect("Failed to load quantized Mistral.")
}

pub fn prompt_ai_agent(words_on_board: Vec<String>, clue: &Clue) -> Vec<String> {
    let prompt = format!(
        "In a game of Codenames, the words on the board are:\n\n\

        {}\n\n\

        Spymaster clue word: {}\n\
        Spymaster clue number: {}\n\n\

        Agent guesses: ",
        words_on_board.join(" "),
        clue.word,
        clue.number
    );
    prompt_mistral(prompt.as_str()).expect("Failed to run pipeline!");
    words_on_board.into_iter().take(clue.number).collect()
}

pub fn prompt_ai_spymaster(team_words: Vec<String>, non_team_words: Vec<String>) -> Clue {
    Clue {
        word: String::from("hello"),
        number: 1,
    }
}

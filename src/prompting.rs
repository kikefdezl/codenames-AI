use candle_core::quantized::gguf_file;
use candle_core::{Device, Tensor};
use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_llama as model;
use model::ModelWeights;
use std::io::Write;
use tokenizers::Tokenizer;

use crate::models::base::Model;
use crate::models::mistral::Mistral7BInstruct;
use crate::settings;
use crate::utils::Clue;

pub fn prompt_ai_agent(words_on_board: Vec<String>, clue: &Clue) -> Vec<String> {
    let prompt = format!(
        "In a game of Codenames, the words on the board are:\n\n

        {}\n\n

        Spymaster clue word: {}\n\
        Spymaster clue number: {}\n\n

        Agent guesses: ",
        words_on_board.join(" "),
        clue.word,
        clue.number
    );
    prompt_model(prompt.as_str()).expect("Failed to run pipeline!");
    words_on_board.into_iter().take(clue.number).collect()
}

pub fn prompt_ai_spymaster(team_words: Vec<String>, non_team_words: Vec<String>) -> Clue {
    Clue {
        word: String::from("hello"),
        number: 1,
    }
}

fn select_model(model: &str) -> Box<dyn Model> {
    match model {
        "Mistral7bInstruct" => return Box::new(Mistral7BInstruct),
        _ => panic!("Invalid model selected!"),
    }
}

pub fn prompt_model(prompt: &str) -> anyhow::Result<()> {
    let api = hf_hub::api::sync::Api::new()?;
    let model_spec = select_model("Mistral7bInstruct");
    let api = api.model(model_spec.model_repo());
    let model_path = api.get(&model_spec.model_filename())?;
    let mut file = std::fs::File::open(&model_path)?;

    let model = gguf_file::Content::read(&mut file).map_err(|e| e.with_path(model_path))?;
    let mut total_size_in_bytes = 0;
    for (_, tensor) in model.tensor_infos.iter() {
        let elem_count = tensor.shape.elem_count();
        total_size_in_bytes +=
            elem_count * tensor.ggml_dtype.type_size() / tensor.ggml_dtype.blck_size();
    }
    let mut model = ModelWeights::from_gguf(model, &mut file)?;
    println!("Model built!");

    let api = hf_hub::api::sync::Api::new()?;
    let api = api.model(model_spec.tokenizer_repo());
    let tokenizer_path = api.get(&model_spec.tokenizer_filename())?;
    let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(anyhow::Error::msg)?;

    let mut tos = TokenOutputStream::new(tokenizer);

    let pre_prompt_tokens = vec![];

    loop {
        print!("{}", &prompt);
        let tokens = tos
            .tokenizer()
            .encode(prompt, true)
            .map_err(anyhow::Error::msg)?;
        let prompt_tokens = [&pre_prompt_tokens, tokens.get_ids()].concat();
        let to_sample = settings::SAMPLE_LEN.saturating_sub(1);
        let prompt_tokens = if prompt_tokens.len() + to_sample > model::MAX_SEQ_LEN - 10 {
            let to_remove = prompt_tokens.len() + to_sample + 10 - model::MAX_SEQ_LEN;
            prompt_tokens[prompt_tokens.len().saturating_sub(to_remove)..].to_vec()
        } else {
            prompt_tokens
        };

        let mut all_tokens = vec![];
        let mut logits_processor =
            LogitsProcessor::new(settings::SEED, settings::TEMPERATURE, settings::TOP_P);

        let mut next_token = {
            let input = Tensor::new(prompt_tokens.as_slice(), &Device::Cpu)?.unsqueeze(0)?;
            let logits = model.forward(&input, 0)?;
            let logits = logits.squeeze(0)?;
            logits_processor.sample(&logits)?
        };
        all_tokens.push(next_token);
        if let Some(t) = tos.next_token(next_token)? {
            print!("{t}");
            std::io::stdout().flush()?;
        }

        let eos_token = "</s>";
        let eos_token = *tos.tokenizer().get_vocab(true).get(eos_token).unwrap();

        for index in 0..to_sample {
            let input = Tensor::new(&[next_token], &Device::Cpu)?.unsqueeze(0)?;
            let logits = model.forward(&input, prompt_tokens.len() + index)?;
            let logits = logits.squeeze(0)?;
            let logits = if settings::REPEAT_PENALTY == 1. {
                logits
            } else {
                let start_at = all_tokens.len().saturating_sub(settings::REPEAT_LAST_N);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    settings::REPEAT_PENALTY,
                    &all_tokens[start_at..],
                )?
            };
            next_token = logits_processor.sample(&logits)?;
            all_tokens.push(next_token);
            if let Some(t) = tos.next_token(next_token)? {
                print!("{t}");
                std::io::stdout().flush()?;
            }
            if next_token == eos_token {
                break;
            };
        }

        if let Some(rest) = tos.decode_rest().map_err(candle_core::Error::msg)? {
            print!("{rest}");
        }
        std::io::stdout().flush()?;
        break;
    }

    Ok(())
}

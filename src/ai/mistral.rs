use candle_core::quantized::gguf_file;
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use std::io::Write;
use tokenizers::Tokenizer;

use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::models::quantized_llama as model;
use model::ModelWeights;

const MODEL_REPO: &str = "TheBloke/Mistral-7B-Instruct-v0.1-GGUF";
const MODEL_FILENAME: &str = "mistral-7b-instruct-v0.1.Q4_0.gguf";
const TOKENIZER_REPO: &str = "mistralai/Mistral-7B-v0.1";
const TOKENIZER_FILENAME: &str = "tokenizer.json";
const TEMPERATURE: Option<f64> = Some(0.1);
const REPEAT_PENALTY: f32 = 1.1;
const REPEAT_LAST_N: usize = 64;
const SAMPLE_LEN: usize = 1000;
const SEED: u64 = 299792458;
const TOP_P: Option<f64> = None;

fn format_size(size_in_bytes: usize) -> String {
    if size_in_bytes < 1_000 {
        format!("{}B", size_in_bytes)
    } else if size_in_bytes < 1_000_000 {
        format!("{:.2}KB", size_in_bytes as f64 / 1e3)
    } else if size_in_bytes < 1_000_000_000 {
        format!("{:.2}MB", size_in_bytes as f64 / 1e6)
    } else {
        format!("{:.2}GB", size_in_bytes as f64 / 1e9)
    }
}

pub fn prompt_mistral(prompt: &str) -> anyhow::Result<()> {
    let api = hf_hub::api::sync::Api::new()?;
    let api = api.model(MODEL_REPO.to_string());
    let model_path = api.get(MODEL_FILENAME)?;
    let mut file = std::fs::File::open(&model_path)?;

    let model = gguf_file::Content::read(&mut file).map_err(|e| e.with_path(model_path))?;
    let mut total_size_in_bytes = 0;
    for (_, tensor) in model.tensor_infos.iter() {
        let elem_count = tensor.shape.elem_count();
        total_size_in_bytes +=
            elem_count * tensor.ggml_dtype.type_size() / tensor.ggml_dtype.blck_size();
    }
    println!(
        "loaded {:?} tensors ({})",
        model.tensor_infos.len(),
        &format_size(total_size_in_bytes),
    );
    let mut model = ModelWeights::from_gguf(model, &mut file)?;
    println!("Model built!");

    let api = hf_hub::api::sync::Api::new()?;
    let api = api.model(TOKENIZER_REPO.to_string());
    let tokenizer_path = api.get(TOKENIZER_FILENAME)?;
    let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(anyhow::Error::msg)?;

    let mut tos = TokenOutputStream::new(tokenizer);
    let prompt = format!("<s>[INST] {} [/INST]", prompt).to_string();

    let pre_prompt_tokens = vec![];

    loop {
        print!("{}", &prompt);
        let tokens = tos
            .tokenizer()
            .encode(prompt, true)
            .map_err(anyhow::Error::msg)?;
        let prompt_tokens = [&pre_prompt_tokens, tokens.get_ids()].concat();
        let to_sample = SAMPLE_LEN.saturating_sub(1);
        let prompt_tokens = if prompt_tokens.len() + to_sample > model::MAX_SEQ_LEN - 10 {
            let to_remove = prompt_tokens.len() + to_sample + 10 - model::MAX_SEQ_LEN;
            prompt_tokens[prompt_tokens.len().saturating_sub(to_remove)..].to_vec()
        } else {
            prompt_tokens
        };

        let mut all_tokens = vec![];
        let mut logits_processor = LogitsProcessor::new(SEED, TEMPERATURE, TOP_P);

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
            let logits = if REPEAT_PENALTY == 1. {
                logits
            } else {
                let start_at = all_tokens.len().saturating_sub(REPEAT_LAST_N);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    REPEAT_PENALTY,
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

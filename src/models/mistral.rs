use crate::models::base::Model;

pub struct Mistral7BInstruct;

impl Model for Mistral7BInstruct {
    fn model_repo(self: &Self) -> String {
        String::from("TheBloke/Mistral-7B-Instruct-v0.1-GGUF")
    }
    fn model_filename(self: &Self) -> String {
        String::from("mistral-7b-instruct-v0.1.Q4_0.gguf")
    }
    fn tokenizer_repo(self: &Self) -> String {
        String::from("mistralai/Mistral-7B-v0.1")
    }
    fn tokenizer_filename(self: &Self) -> String {
        String::from("tokenizer.json")
    }
    fn prompt_template(self: &Self) -> String {
        "<s>[INST] {} [/INST]".to_string()
    }
}

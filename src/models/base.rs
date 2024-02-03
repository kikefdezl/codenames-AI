pub trait Model {
    fn model_filename(self: &Self) -> String;
    fn tokenizer_repo(self: &Self) -> String;
    fn tokenizer_filename(self: &Self) -> String;
    fn model_repo(self: &Self) -> String;
    fn prompt_template(self: &Self) -> String;
}

use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Corpus {
    pub corpus: String,
    pub symbols: HashMap<char, f32>,
    pub digrams: HashMap<String, f32>,
    pub trigrams: HashMap<String, f32>,
}

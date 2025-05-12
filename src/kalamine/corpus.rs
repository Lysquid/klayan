use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Corpus {
    pub path: String,
    pub symbols: HashMap<char, f32>,
    pub digrams: HashMap<[char; 2], f32>,
    pub trigrams: HashMap<[char; 3], f32>,
}

fn ngram_to_char(map: HashMap<Ngram<1>, f32>) -> HashMap<char, f32> {
    map.into_iter()
        .map(|(ngram, value)| (ngram.0[0], value))
        .collect()
}

fn ngram_to_char_array<const N: usize>(map: HashMap<Ngram<N>, f32>) -> HashMap<[char; N], f32> {
    map.into_iter()
        .map(|(ngram, value)| (ngram.0, value))
        .collect()
}

impl<'de> serde::Deserialize<'de> for Corpus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let corpus = CorpusJSON::deserialize(d)?;

        Ok(Corpus {
            path: corpus.corpus,
            symbols: ngram_to_char(corpus.symbols),
            digrams: ngram_to_char_array(corpus.digrams),
            trigrams: ngram_to_char_array(corpus.trigrams),
        })
    }
}

// The point of having an struct with wrapper for data is to do the validation
// during deserialization, to have helpful error messages with the line/column number
#[derive(Debug, serde::Deserialize)]
struct CorpusJSON {
    corpus: String,
    symbols: HashMap<Ngram<1>, f32>,
    digrams: HashMap<Ngram<2>, f32>,
    trigrams: HashMap<Ngram<3>, f32>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Ngram<const N: usize>(pub [char; N]);

impl<'de, const N: usize> serde::Deserialize<'de> for Ngram<N> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let chars: Vec<char> = s.chars().collect();

        if chars.len() != N {
            return Err(serde::de::Error::custom(format!(
                "expected {N} characters, got {}",
                chars.len()
            )));
        }

        let symbols: [char; N] = chars.try_into().unwrap();
        Ok(Ngram(symbols))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn deserialize_corpus() {
        let json = r#"{
            "corpus": "text.txt",
            "symbols": {
                "a": 9.0,
                "b": 8.0,
                "c": 7.0
            },
            "digrams": {
                "ab": 6.0,
                "bc": 5.0
            },
            "trigrams": {
                "abc": 4.0
            }
        }"#;
        let corpus: Corpus = serde_json::from_str(json).unwrap();
        let expected = Corpus {
            path: String::from("text.txt"),
            symbols: HashMap::from([('a', 9.0), ('b', 8.0), ('c', 7.0)]),
            digrams: HashMap::from([(['a', 'b'], 6.0), (['b', 'c'], 5.0)]),
            trigrams: HashMap::from([(['a', 'b', 'c'], 4.0)]),
        };
        assert_eq!(corpus, expected);
    }

    #[test]
    fn deserialize_corpus_empty() {
        let json = r#"{
            "corpus": "",
            "symbols": {},
            "digrams": {},
            "trigrams": {}
        }"#;
        let corpus: Corpus = serde_json::from_str(json).unwrap();
        let expected = Corpus {
            path: String::new(),
            symbols: HashMap::new(),
            digrams: HashMap::new(),
            trigrams: HashMap::new(),
        };
        assert_eq!(corpus, expected);
    }

    #[test]
    fn deserialize_corpus_invalid_ngram() {
        let json = r#"{
            "corpus": "text.txt",
            "symbols": {
                "ab": 9.0
            },
            "digrams": {},
            "trigrams": {}
        }"#;
        let result: Result<Corpus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}

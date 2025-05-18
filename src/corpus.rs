use std::collections::HashMap;

use crate::keyseq::KeySymbol;

pub fn keysym_ngram_freq<const N: usize>(
    ngrams_freq: &HashMap<[char; N], f32>,
    sym_to_keystrokes: &HashMap<char, Vec<KeySymbol>>,
) -> Vec<([KeySymbol; N], f32)> {
    let mut ngram_map: HashMap<[KeySymbol; N], f32> = HashMap::new();

    'outer: for (ngram, freq) in ngrams_freq {
        let mut ngram_key_seq: Vec<&KeySymbol> = Vec::new();
        for c in ngram {
            let sym_key_seq = match sym_to_keystrokes.get(&c) {
                Some(key_seq) => key_seq,
                None => continue 'outer,
            };
            ngram_key_seq.extend(sym_key_seq.iter());
        }
        for window in ngram_key_seq.windows(N) {
            if let Ok(tuple) = <[&KeySymbol; N]>::try_from(window) {
                let keysym_ngram: [KeySymbol; N] = tuple.map(|key| key.clone());
                ngram_map
                    .entry(keysym_ngram)
                    .and_modify(|f| *f += freq)
                    .or_insert(*freq);
            }
        }
    }
    ngram_map.into_iter().collect()
}

pub fn keysym_freq(
    char_freq: &HashMap<char, f32>,
    sym_to_keystrokes: &HashMap<char, Vec<KeySymbol>>,
) -> Vec<(KeySymbol, f32)> {
    let char_freq: HashMap<[char; 1], f32> = char_freq.iter().map(|(c, f)| ([*c], *f)).collect();
    let res = keysym_ngram_freq(&char_freq, &sym_to_keystrokes);
    res.iter().map(|(c, f)| (c[0].clone(), *f)).collect()
}

// TODO: test

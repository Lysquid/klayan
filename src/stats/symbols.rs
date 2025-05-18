use std::collections::HashMap;

use crate::hands::Finger;
use crate::kalamine::PhysicalKey;
use crate::stats::utils;

pub fn unsupported_characters<T>(
    corpus_symbols: &HashMap<char, f32>,
    layout_symbols: &HashMap<char, T>,
) -> Vec<(char, f32)> {
    corpus_symbols
        .iter()
        .filter(|(c, _)| !layout_symbols.contains_key(c))
        .map(|(c, f)| (*c, *f))
        .collect()
}

pub fn calc_finger_freq(
    sym_to_keystrokes: &HashMap<char, Vec<PhysicalKey>>,
    sym_freq: &HashMap<char, f32>,
) -> Vec<(Finger, f32)> {
    let mut finger_freq: HashMap<Finger, f32> = HashMap::new();

    for (symbol, freq) in sym_freq.iter() {
        let keystrokes = match sym_to_keystrokes.get(symbol) {
            Some(ks) => ks,
            None => continue,
        };
        for key in keystrokes {
            finger_freq
                .entry(key.finger())
                .and_modify(|f| *f += freq)
                .or_insert(*freq);
        }
    }
    utils::normalize(&mut finger_freq);
    let mut finger_freq: Vec<(_, _)> = finger_freq.into_iter().collect();
    utils::sort_vec_by_key(&mut finger_freq);
    finger_freq
}

#[cfg(test)]
mod tests {

    use super::*;
    use PhysicalKey::*;

    #[test]
    fn test_unsupported_characters() {
        let supported: HashMap<char, f32> = HashMap::from([('a', 1.0), ('é', 1.0)]);
        let unsupported: HashMap<char, f32> = HashMap::from([('b', 1.0)]);
        let layout_symbols: HashMap<char, Vec<PhysicalKey>> = HashMap::from([
            ('a', vec![KeyA]),
            ('c', vec![KeyC]),
            ('é', vec![Quote, KeyE]),
        ]);
        let mut corpus_symbols = supported.clone();
        corpus_symbols.extend(unsupported.iter());
        let expected_unsupported: Vec<(char, f32)> = unsupported.into_iter().collect();

        let result = unsupported_characters(&corpus_symbols, &layout_symbols);
        assert_eq!(result, expected_unsupported);
    }
}

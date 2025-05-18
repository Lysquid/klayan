use std::collections::HashMap;

use crate::keyseq::KeySymbol;

use super::utils;

pub fn symbol_stats(
    corpus_symbols: &HashMap<char, f32>,
    layout_symbols: &HashMap<char, Vec<KeySymbol>>,
) -> SymbolStats {
    let unsupported = unsupported_characters(corpus_symbols, layout_symbols);

    SymbolStats {
        total_unsupported: utils::result_sum(&unsupported),
        list_unsupported: utils::result_vec(unsupported),
    }
}

/// Using a generic because we don't care about actual type,
/// so it is easier to test without having to define everything
fn unsupported_characters<T>(
    corpus_symbols: &HashMap<char, f32>,
    layout_symbols: &HashMap<char, T>,
) -> Vec<(char, f32)> {
    corpus_symbols
        .iter()
        .filter(|(c, _)| !layout_symbols.contains_key(c))
        .map(|(c, f)| (*c, *f))
        .collect()
}

pub struct SymbolStats {
    pub total_unsupported: f32,
    pub list_unsupported: Vec<(char, f32)>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::kalamine::PhysicalKey::{self, *};

    #[test]
    #[rustfmt::skip]
    fn test_unsupported_characters() {
        let supported: HashMap<char, f32> = HashMap::from([
            ('a', 1.0),
            ('é', 1.0),
        ]);
        let unsupported: HashMap<char, f32> = HashMap::from([
            ('b', 1.0),
        ]);
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

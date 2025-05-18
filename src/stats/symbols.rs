use std::collections::HashMap;

use crate::hands::Finger;
use crate::keystrokes::KeySymbol;
use crate::stats::utils;

use super::utils::add_or_insert;

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

pub fn calc_finger_freq(keysym_freq: &Vec<(KeySymbol, f32)>) -> Vec<(Finger, f32)> {
    let mut finger_freq: HashMap<Finger, f32> = Finger::iter_all().map(|f| (f, 0.0)).collect();

    for (keysym, freq) in keysym_freq.iter() {
        add_or_insert(finger_freq.entry(keysym.key.finger()), *freq);
    }
    let mut finger_freq = utils::map_to_vec(finger_freq);
    utils::sort_vec_by_key(&mut finger_freq);
    finger_freq
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

    #[test]
    fn finger_freq() {
        use crate::kalamine::Symbol::Character;
        let keysym_freq: Vec<(KeySymbol, f32)> = Vec::from([
            (KeySymbol::new(Character('q'), KeyQ), 1.0),
            (KeySymbol::new(Character('w'), KeyW), 2.0),
            (KeySymbol::new(Character('e'), KeyE), 3.0),
            (KeySymbol::new(Character('r'), KeyR), 4.0),
            (KeySymbol::new(Character('t'), KeyT), 5.0),
            (KeySymbol::new(Character('y'), KeyY), 6.0),
            (KeySymbol::new(Character('u'), KeyU), 7.0),
            (KeySymbol::new(Character('i'), KeyI), 8.0),
        ]);
        use crate::hands::Finger::*;
        let mut finger_freq: Vec<(Finger, f32)> = Vec::from([
            (Thumb, 0.0),
            (LeftPinky, 1.0),
            (LeftRing, 2.0),
            (LeftMiddle, 3.0),
            (LeftIndex, 9.0),
            (RightIndex, 13.0),
            (RightMiddle, 8.0),
            (RightRing, 0.0),
            (RightPinky, 0.0),
        ]);
        utils::sort_vec_by_key(&mut finger_freq);
        let result = calc_finger_freq(&keysym_freq);
        assert_eq!(result, finger_freq);
    }
}

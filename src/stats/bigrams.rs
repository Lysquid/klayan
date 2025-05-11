use std::collections::hash_map::Entry;
use std::collections::HashMap;

use itertools::Itertools;

use crate::hands::{Finger, Hand, Row};
use crate::kalamine::PhysicalKey;
use crate::keystrokes::Keystrokes;

fn bigram_two_keys_iter(
    sym_to_keystrokes: &HashMap<char, Keystrokes>,
    bigram: [char; 2],
) -> Option<impl Iterator<Item = (&PhysicalKey, &PhysicalKey)>> {
    let ks1 = sym_to_keystrokes.get(&bigram[0])?;
    let ks2 = sym_to_keystrokes.get(&bigram[1])?;
    Some(ks1.iter().chain(ks2.iter()).tuple_windows())
}

pub fn add_freq<K>(entry: Entry<'_, K, f32>, freq: f32) {
    entry.and_modify(|f| *f += freq).or_insert(freq);
}

fn is_lsb(finger1: Finger, finger2: Finger) -> bool {
    use Finger::*;
    // TODO: do a less simplistic implementation with columns once I have the info of the geometry
    match (finger1, finger2) {
        (LeftMiddle, LeftIndex) | (LeftIndex, LeftMiddle) => true,
        (RightMiddle, RightIndex) | (RightIndex, RightMiddle) => true,
        (_, _) => false,
    }
}

fn is_scissors(key1: PhysicalKey, key2: PhysicalKey) -> bool {
    let row1 = Row::from(key1);
    let row2 = Row::from(key2);
    match (Hand::from(key1), Hand::from(key2)) {
        (Hand::Thumbs, _) | (_, Hand::Thumbs)=> false,
        (hand1, hand2) => hand1 != hand2 && row1.as_u32().abs_diff(row2.as_u32()) >= 2,
    }
}

pub fn calc_bigrams(
    sym_to_keystrokes: &HashMap<char, Keystrokes>,
    bigrams_freq: &HashMap<[char; 2], f32>,
) -> (HashMap<[char; 2], f32>, HashMap<[char; 2], f32>) {
    let mut sfb: HashMap<[char; 2], f32> = HashMap::new();
    let mut sku: HashMap<[char; 2], f32> = HashMap::new();
    let mut lsb: HashMap<[char; 2], f32> = HashMap::new();
    let mut scissors: HashMap<[char; 2], f32> = HashMap::new();
    let mut per_finger_sfb: HashMap<Finger, f32> = HashMap::new();
    let mut per_finger_sku: HashMap<Finger, f32> = HashMap::new();

    for (&bigram, &freq) in bigrams_freq {
        let iter = match bigram_two_keys_iter(sym_to_keystrokes, bigram) {
            Some(iter) => iter,
            None => continue,
        };
        for (&key1, &key2) in iter {
            let finger1 = Finger::from(key1);
            let finger2 = Finger::from(key2);
            if key1 == key2 {
                add_freq(sku.entry(bigram), freq);
                add_freq(per_finger_sku.entry(finger1), freq);
            } else if finger1 == finger2 {
                add_freq(sfb.entry(bigram), freq);
                add_freq(per_finger_sfb.entry(finger1), freq);
            } else if is_lsb(finger1, finger2) {
                add_freq(lsb.entry(bigram), freq);
            }
            if is_scissors(key1, key2) {
                add_freq(scissors.entry(bigram), freq);
            }
        }
    }
    (sfb, sku)
}

#[cfg(test)]
#[rustfmt::skip] 
mod tests {

    use super::*;
    use crate::kalamine::PhysicalKey::*;

    #[test]
    fn clac_bigrams_simple() {
        let sym_to_ks = HashMap::from([
            ('e', vec![KeyE]),
            ('r', vec![KeyR]),
            ('d', vec![KeyD]),
        ]);
        let expected_sfb = HashMap::from([
            (['e','d'], 1.0),
            (['d','e'], 1.0),
        ]);
        let expected_sku = HashMap::from([
            (['r','r'], 1.0),
        ]);
        let other_bigrams = HashMap::from([
            (['e','r'], 1.0),
            (['r','e'], 1.0),
            (['r','d'], 1.0),
            (['d','r'], 1.0),
        ]);
        let mut bigrams_freq = other_bigrams.clone();
        bigrams_freq.extend(expected_sfb.iter());
        bigrams_freq.extend(expected_sku.iter());
        let (sfb, sku) = calc_bigrams(&sym_to_ks, &bigrams_freq);
        assert_eq!(sfb, expected_sfb);
        assert_eq!(sku, expected_sku);
    }

    #[test]
    fn clac_bigrams_deadkey() {
        let sym_to_ks = HashMap::from([
            ('d', vec![KeyD]),
            ('e', vec![KeyE]),
            ('é', vec![Quote, KeyE]),
        ]);
        let expected_sfb = HashMap::from([
            (['é','d'], 1.0),
        ]);
        let expected_sku = HashMap::from([
            (['é', 'e'], 1.0),
        ]);
        let other_bigrams = HashMap::from([
            (['d','é'], 1.0),
            (['é','e'], 1.0),
            (['é','é'], 1.0),
        ]);
        let mut bigrams_freq = other_bigrams.clone();
        bigrams_freq.extend(expected_sfb.clone());
        bigrams_freq.extend(expected_sku.clone());
        let (sfb, sku) = calc_bigrams(&sym_to_ks, &bigrams_freq);
        assert_eq!(sfb, expected_sfb);
        assert_eq!(sku, expected_sku);
    }

    #[test]
    fn clac_bigrams_double_deadkey() {
        let sym_to_ks = HashMap::from([
            ('e', vec![KeyE]),
            ('ë', vec![Quote, Quote, KeyE]),
            ('r', vec![KeyR]),
        ]);
        let mut expected_sku = HashMap::from([
            (['ë', 'r'], 1.0), // sku with double dead key
            (['ë', 'e'], 1.0),
            (['ë', 'ë'], 1.0),
        ]);
        let other_bigrams = HashMap::from([
            (['e','r'], 1.0),
        ]);
        let mut bigrams_freq = other_bigrams.clone();
        bigrams_freq.extend(expected_sku.clone());
        let (_, sku) = calc_bigrams(&sym_to_ks, &bigrams_freq);
        *expected_sku.get_mut(&['ë', 'e']).unwrap() = 2.0; // ' ' e e => 2 sku
        *expected_sku.get_mut(&['ë', 'ë']).unwrap() = 2.0; // ' ' e ' ' e => 2 sku
        assert_eq!(sku, expected_sku);
    }

}

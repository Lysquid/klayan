use crate::geometry::{Geometry, Row, U};
use crate::hands::{Finger, Hand};
use crate::kalamine::PhysicalKey;
use crate::keystrokes::Keystrokes;
use std::collections::HashMap;

use super::utils::{add_or_insert, bigram_two_keys_iter};

pub fn calc_bigrams(
    sym_to_keystrokes: &HashMap<char, Keystrokes>,
    bigrams_freq: &HashMap<[char; 2], f32>,
    geometry: Geometry,
) -> (HashMap<[char; 2], f32>, HashMap<[char; 2], f32>) {
    let mut sfb: HashMap<[char; 2], f32> = HashMap::new();
    let mut sku: HashMap<[char; 2], f32> = HashMap::new();
    let mut per_finger_sfb: HashMap<Finger, f32> = HashMap::new();
    let mut per_finger_sku: HashMap<Finger, f32> = HashMap::new();
    let mut lsb: HashMap<[char; 2], f32> = HashMap::new();
    let mut scissors: HashMap<[char; 2], f32> = HashMap::new();

    for (&bigram, &freq) in bigrams_freq {
        let iter = match bigram_two_keys_iter(sym_to_keystrokes, bigram) {
            Some(iter) => iter,
            None => continue,
        };
        for (&key1, &key2) in iter {
            let finger1 = Finger::from(key1);
            let finger2 = Finger::from(key2);
            if is_sku(key1, key2) {
                add_or_insert(sku.entry(bigram), freq);
                add_or_insert(per_finger_sku.entry(finger1), freq);
            } else if is_sfb_finger(finger1, finger2) {
                add_or_insert(sfb.entry(bigram), freq);
                add_or_insert(per_finger_sfb.entry(finger1), freq);
            } else if is_lsb(key1, key2, geometry) {
                add_or_insert(lsb.entry(bigram), freq);
            }
            if is_scissors(key1, key2) {
                add_or_insert(scissors.entry(bigram), freq);
            }
        }
    }
    // TODO: return a struct BigramsAnalysis
    // and complete it with the total of each stat
    (sfb, sku)
}

pub fn is_sku(key1: PhysicalKey, key2: PhysicalKey) -> bool {
    key1 == key2
}

pub fn is_sfb(key1: PhysicalKey, key2: PhysicalKey) -> bool {
    // TODO: depend on geometry
    is_sfb_finger(Finger::from(key1), Finger::from(key2))
}

fn is_sfb_finger(finger1: Finger, finger2: Finger) -> bool {
    finger1 == finger2
}

/// Using Keyboard layout doc definition
/// https://docs.google.com/document/d/1W0jhfqJI2ueJ2FNseR4YAFpNfsUM-_FlREHbpNGmC2o/edit?tab=t.i8oe0bwffr95
pub fn is_lsb(key1: PhysicalKey, key2: PhysicalKey, geometry: Geometry) -> bool {
    let finger1 = Finger::from(key1);
    let finger2 = Finger::from(key2);
    let finger_dist = match finger1.distance(&finger2) {
        Some(dist) => dist,
        None => return false,
    };
    let horizontal_dist = match geometry.lateral_distance(key1, key2) {
        Some(dist) => dist,
        None => return false,
    };
    match finger_dist {
        1 => horizontal_dist >= 2 * U,         // adjacent fingers (2U)
        2 => horizontal_dist >= 3 * U + U / 2, // semi-adjacent fingers (3.5U)
        _ => false,
    }
}

pub fn is_scissors(key1: PhysicalKey, key2: PhysicalKey) -> bool {
    // TODO: use defintion from from keyboard layout doc (FSB, HSF)
    let row1 = Row::from(key1);
    let row2 = Row::from(key2);
    match (Hand::from(key1), Hand::from(key2)) {
        (Hand::Thumbs, _) | (_, Hand::Thumbs) => false,
        (hand1, hand2) => hand1 == hand2 && row1.distance(&row2) >= 2,
    }
}

#[cfg(test)]
#[rustfmt::skip] 
mod tests {

    use super::*;
    use crate::kalamine::PhysicalKey::*;

    #[test]
    fn lsb() {
        assert!(!is_lsb(KeyQ, KeyT, Geometry::Ortho));
        assert!(!is_lsb(KeyW, KeyT, Geometry::Ortho));
        assert!(is_lsb(KeyE, KeyT, Geometry::Ortho));
        assert!(!is_lsb(KeyR, KeyT, Geometry::Ortho));
        assert!(!is_lsb(KeyT, KeyT, Geometry::Ortho));
        assert!(!is_lsb(KeyY, KeyT, Geometry::Ortho));
        
        assert!(!is_lsb(KeyQ, KeyB, Geometry::ANSI));
        assert!(is_lsb(KeyW, KeyB, Geometry::ANSI)); // LSB due to stagger
        assert!(is_lsb(KeyE, KeyB, Geometry::ANSI)); 
        assert!(!is_lsb(KeyR, KeyB, Geometry::ANSI));
        assert!(!is_lsb(KeyT, KeyB, Geometry::ANSI));

        assert!(!is_lsb(KeyW, KeyG, Geometry::ANSI)); // stagger not big enough
        
        assert!(!is_lsb(KeyH, Quote, Geometry::ANSI));
        assert!(!is_lsb(KeyJ, Quote, Geometry::ANSI));
        assert!(!is_lsb(KeyK, Quote, Geometry::ANSI));
        assert!(is_lsb(KeyL, Quote, Geometry::ANSI));
        assert!(!is_lsb(Semicolon, Quote, Geometry::ANSI));
    }

    #[test]
    fn scissors() {
        assert!(is_scissors(Digit3, KeyV));
        assert!(is_scissors(KeyE, KeyV));
        assert!(!is_scissors(KeyD, KeyV));
        assert!(!is_scissors(KeyC, KeyV));
        assert!(!is_scissors(KeyV, KeyV));
        assert!(!is_scissors(KeyF, KeyJ));
    }

    // TODO: remove those tests or make them integration tests
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
        let (sfb, sku) = calc_bigrams(&sym_to_ks, &bigrams_freq, Geometry::ISO);
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
        let (sfb, sku) = calc_bigrams(&sym_to_ks, &bigrams_freq, Geometry::ISO);
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
        let (_, sku) = calc_bigrams(&sym_to_ks, &bigrams_freq, Geometry::ISO);
        *expected_sku.get_mut(&['ë', 'e']).unwrap() = 2.0; // ' ' e e => 2 sku
        *expected_sku.get_mut(&['ë', 'ë']).unwrap() = 2.0; // ' ' e ' ' e => 2 sku
        assert_eq!(sku, expected_sku);
    }

}

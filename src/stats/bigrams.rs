use crate::geometry::{Geometry, Row, U};
use crate::hands::{Finger, Hand, RollDirection};
use crate::kalamine::{PhysicalKey, Symbol};
use crate::keyseq::KeySymbol;
use std::collections::HashMap;

use super::utils::{self, add_or_insert};

type Bigram = [Symbol; 2];

pub fn bigram_stats(
    bigrams_freq: &Vec<([KeySymbol; 2], f32)>,
    geometry: Geometry,
) -> BigramStats {
    let mut sfb: Vec<(Bigram, f32)> = Vec::new();
    let mut sku: Vec<(Bigram, f32)> = Vec::new();
    let mut per_finger_sfb: HashMap<Finger, f32> = HashMap::new();
    let mut per_finger_sku: HashMap<Finger, f32> = HashMap::new();
    let mut in_rolls: Vec<(Bigram, f32)> = Vec::new();
    let mut out_rolls: Vec<(Bigram, f32)> = Vec::new();
    let mut lsb: Vec<(Bigram, f32)> = Vec::new();
    let mut scissors: Vec<(Bigram, f32)> = Vec::new();

    for (bigram_keys, freq) in bigrams_freq {
        let bigram = [bigram_keys[0].symbol(), bigram_keys[1].symbol()];
        let freq = *freq;
        let key1 = bigram_keys[0].key;
        let key2 = bigram_keys[1].key;

        if key1 == key2 {
            sku.push((bigram, freq));
            add_or_insert(per_finger_sku.entry(key1.finger()), freq);
        } else if key1.finger() == key2.finger() {
            sfb.push((bigram, freq));
            add_or_insert(per_finger_sfb.entry(key1.finger()), freq);
        } else {
            if is_in_roll(key1, key2) {
                in_rolls.push((bigram, freq));
            } else if is_out_roll(key1, key2) {
                out_rolls.push((bigram, freq));
            }
            if is_lsb(key1, key2, geometry) {
                lsb.push((bigram, freq));
            }
            if is_scissors(key1, key2) {
                scissors.push((bigram, freq));
            }
        }
    }

    BigramStats {
        total_sku: utils::result_sum(&sku),
        total_sfb: utils::result_sum(&sfb),
        total_lsb: utils::result_sum(&lsb),
        total_scissors: utils::result_sum(&scissors),
        total_in_rolls: utils::result_sum(&in_rolls),
        total_out_rolls: utils::result_sum(&out_rolls),
        per_finger_sku: utils::result_vec_from_map(per_finger_sku),
        per_finger_sfb: utils::result_vec_from_map(per_finger_sfb),
        list_sku: utils::result_vec(sku),
        list_sfb: utils::result_vec(sfb),
        list_lsb: utils::result_vec(lsb),
        list_in_rolls: utils::result_vec(in_rolls),
        list_out_rolls: utils::result_vec(out_rolls),
        list_scissors: utils::result_vec(scissors),
    }
}

/// Using Keyboard layout doc definition
/// https://docs.google.com/document/d/1W0jhfqJI2ueJ2FNseR4YAFpNfsUM-_FlREHbpNGmC2o/edit?tab=t.i8oe0bwffr95
pub fn is_lsb(key1: PhysicalKey, key2: PhysicalKey, geometry: Geometry) -> bool {
    let finger_dist = match Finger::distance(key1.finger(), key2.finger()) {
        Some(dist) => dist,
        None => return false,
    };
    let horizontal_dist = match geometry.horizontal_distance(key1, key2) {
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
    // TODO: do a less simplistic implementation
    // Later on, use definition from from keyboard layout doc (FSB, HSF)
    match (key1.hand(), key2.hand()) {
        (Hand::Thumbs, _) | (_, Hand::Thumbs) => false,
        (hand1, hand2) => hand1 == hand2 && Row::distance(key1.row(), key2.row()) >= 2,
    }
}

pub fn is_in_roll(key1: PhysicalKey, key2: PhysicalKey) -> bool {
    let finger1 = key1.finger();
    let finger2 = key2.finger();
    finger1.roll_direction(finger2) == RollDirection::Inside
}

pub fn is_out_roll(key1: PhysicalKey, key2: PhysicalKey) -> bool {
    let finger1 = key1.finger();
    let finger2 = key2.finger();
    finger1.roll_direction(finger2) == RollDirection::Outside
}

pub struct BigramStats {
    pub total_sku: f32,
    pub total_sfb: f32,
    pub total_lsb: f32,
    pub total_scissors: f32,
    pub total_in_rolls: f32,
    pub total_out_rolls: f32,
    pub per_finger_sku: Vec<(Finger, f32)>,
    pub per_finger_sfb: Vec<(Finger, f32)>,
    pub list_sku: Vec<(Bigram, f32)>,
    pub list_sfb: Vec<(Bigram, f32)>,
    pub list_lsb: Vec<(Bigram, f32)>,
    pub list_in_rolls: Vec<(Bigram, f32)>,
    pub list_out_rolls: Vec<(Bigram, f32)>,
    pub list_scissors: Vec<(Bigram, f32)>,
}

#[cfg(test)]
#[rustfmt::skip] 
mod tests {

    use super::*;
    use crate::kalamine::PhysicalKey::*;
    use crate::geometry::Geometry::*;

    #[test]
    fn in_roll() {
        assert!(is_in_roll(KeyD, KeyF));
        assert!(!is_in_roll(KeyF, KeyD));
        assert!(is_in_roll(KeyK, KeyJ));
        assert!(!is_in_roll(KeyJ, KeyK));
        assert!(!is_in_roll(KeyA, Space));
    }

    #[test]
    fn out_roll() {
        assert!(!is_out_roll(KeyD, KeyF));
        assert!(is_out_roll(KeyF, KeyD));
        assert!(!is_out_roll(KeyK, KeyJ));
        assert!(is_out_roll(KeyJ, KeyK));
        assert!(!is_out_roll(KeyB, Space));
    }

    #[test]
    fn lsb() {
        assert!(!is_lsb(KeyQ, KeyT, Ortho));
        assert!(!is_lsb(KeyW, KeyT, Ortho));
        assert!(is_lsb(KeyE, KeyT, Ortho)); // Middle-Index LSB
        assert!(!is_lsb(KeyR, KeyT, Ortho));
        assert!(!is_lsb(KeyT, KeyT, Ortho));
        assert!(!is_lsb(KeyY, KeyT, Ortho));
        
        assert!(!is_lsb(KeyQ, KeyB, ANSI));
        assert!(is_lsb(KeyW, KeyB, ANSI)); // LSB due to stagger
        assert!(is_lsb(KeyE, KeyB, ANSI)); 
        assert!(!is_lsb(KeyR, KeyB, ANSI));
        assert!(!is_lsb(KeyT, KeyB, ANSI));

        assert!(!is_lsb(KeyW, KeyG, ANSI)); // stagger not big enough
        
        assert!(!is_lsb(KeyH, Quote, ANSI));
        assert!(!is_lsb(KeyJ, Quote, ANSI));
        assert!(!is_lsb(KeyK, Quote, ANSI));
        assert!(is_lsb(KeyL, Quote, ANSI)); // Ring-Pinky LSB
        assert!(!is_lsb(Semicolon, Quote, ANSI));
        
        assert!(!is_lsb(KeyA, Space, Ortho));
    }

    #[test]
    fn scissors() {
        assert!(is_scissors(Digit3, KeyV));
        assert!(is_scissors(KeyE, KeyV));
        assert!(!is_scissors(KeyD, KeyV));
        assert!(!is_scissors(KeyC, KeyV));
        assert!(!is_scissors(KeyV, KeyV));
        assert!(!is_scissors(KeyF, KeyJ));
        assert!(!is_scissors(Digit4, Space));
    }

}

use std::collections::HashMap;

use super::utils::add_or_insert;
use crate::hands::{Finger, Hand};
use crate::kalamine::PhysicalKey;
use crate::keyseq::KeySymbol;
use crate::stats::utils;
use strum::IntoEnumIterator;

pub fn unigram_stats(keysym_freq: &Vec<(KeySymbol, f32)>) -> UnigramStats {
    let mut key_usage: HashMap<PhysicalKey, f32> = PhysicalKey::iter().map(|k| (k, 0.0)).collect();
    let mut finger_usage: HashMap<Finger, f32> = Finger::iter().map(|f| (f, 0.0)).collect();
    let mut hand_usage: HashMap<Hand, f32> = Hand::iter().map(|h| (h, 0.0)).collect();

    for (keysym, freq) in keysym_freq.iter() {
        let key = keysym.key;
        let freq = *freq;
        add_or_insert(key_usage.entry(key), freq);
        add_or_insert(finger_usage.entry(key.finger()), freq);
        add_or_insert(hand_usage.entry(key.hand()), freq);
    }

    UnigramStats {
        key_usage: utils::result_vec_sorted_by_key(key_usage.into_iter().collect()),
        finger_usage: utils::result_vec_sorted_by_key(finger_usage.into_iter().collect()),
        hand_usage: utils::result_vec_sorted_by_key(hand_usage.into_iter().collect()),
    }
}

pub struct UnigramStats {
    pub key_usage: Vec<(PhysicalKey, f32)>,
    pub finger_usage: Vec<(Finger, f32)>,
    pub hand_usage: Vec<(Hand, f32)>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::kalamine::PhysicalKey::*;

    #[test]
    fn test_unigram_stats() {
        use crate::kalamine::Symbol::Character;
        let keysym_freq: Vec<(KeySymbol, f32)> = Vec::from([
            (KeySymbol::new(Character('q'), KeyQ), 0.01),
            (KeySymbol::new(Character('w'), KeyW), 0.02),
            (KeySymbol::new(Character('e'), KeyE), 0.03),
            (KeySymbol::new(Character('é'), KeyE), 0.04),
            (KeySymbol::new(Character('r'), KeyR), 0.05),
            (KeySymbol::new(Character('t'), KeyT), 0.06),
            (KeySymbol::new(Character('y'), KeyY), 0.07),
            (KeySymbol::new(Character('u'), KeyU), 0.08),
            (KeySymbol::new(Character('i'), KeyI), 0.09),
        ]);
        let key_usage: Vec<(PhysicalKey, f32)> = Vec::from([
            (Space, 0.0),
            (KeyQ, 1.0),
            (KeyW, 2.0),
            (KeyE, 7.0),
            (KeyR, 5.0),
            (KeyT, 6.0),
            (KeyY, 7.0),
            (KeyU, 8.0),
            (KeyI, 9.0),
        ]);
        use crate::hands::Finger::*;
        let finger_usage: Vec<(Finger, f32)> = Vec::from([
            (Thumb, 0.0),
            (LeftPinky, 1.0),
            (LeftRing, 2.0),
            (LeftMiddle, 7.0),
            (LeftIndex, 11.0),
            (RightIndex, 15.0),
            (RightMiddle, 9.0),
            (RightRing, 0.0),
            (RightPinky, 0.0),
        ]);
        let hand_usage: Vec<(Hand, f32)> =
            Vec::from([(Hand::Left, 21.0), (Hand::Right, 24.0), (Hand::Thumbs, 0.0)]);
        let result = unigram_stats(&keysym_freq);
        assert_eq!(utils::round_result_vec(result.finger_usage), finger_usage);
        assert_eq!(utils::round_result_vec(result.hand_usage), hand_usage);
        let result_key_usage = utils::round_result_vec(result.key_usage);
        for elem in key_usage.iter() {
            assert!(result_key_usage.contains(elem));
        }
    }
}

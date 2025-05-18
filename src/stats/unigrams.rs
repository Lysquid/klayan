use std::collections::HashMap;

use crate::hands::{Finger, Hand};
use crate::kalamine::PhysicalKey;
use crate::keyseq::KeySymbol;
use crate::stats::utils;

use super::utils::add_or_insert;

pub fn unigram_stats(keysym_freq: &Vec<(KeySymbol, f32)>) -> UnigramStats {
    let mut finger_usage: HashMap<Finger, f32> = Finger::iter_all().map(|f| (f, 0.0)).collect();

    for (keysym, freq) in keysym_freq.iter() {
        add_or_insert(finger_usage.entry(keysym.key.finger()), *freq);
    }
    let mut finger_usage = utils::map_to_vec(finger_usage);
    utils::sort_vec_by_key(&mut finger_usage);

    UnigramStats {
        key_usage: Vec::new(),
        finger_usage: utils::result_vec_sorted_by_key(finger_usage),
        hand_usage: Vec::new(),
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
    fn finger_usage() {
        use crate::kalamine::Symbol::Character;
        let keysym_freq: Vec<(KeySymbol, f32)> = Vec::from([
            (KeySymbol::new(Character('q'), KeyQ), 0.01),
            (KeySymbol::new(Character('w'), KeyW), 0.02),
            (KeySymbol::new(Character('e'), KeyE), 0.03),
            (KeySymbol::new(Character('r'), KeyR), 0.04),
            (KeySymbol::new(Character('t'), KeyT), 0.05),
            (KeySymbol::new(Character('y'), KeyY), 0.06),
            (KeySymbol::new(Character('u'), KeyU), 0.07),
            (KeySymbol::new(Character('i'), KeyI), 0.08),
        ]);
        use crate::hands::Finger::*;
        let mut finger_usage: Vec<(Finger, f32)> = Vec::from([
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
        utils::sort_vec_by_key(&mut finger_usage);
        let result = unigram_stats(&keysym_freq);
        assert_eq!(result.finger_usage, finger_usage);
    }
}

use std::collections::HashMap;

use crate::hands::Finger;
use crate::kalamine::PhysicalKey;
use crate::stats::utils;

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

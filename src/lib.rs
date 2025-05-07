pub mod hands;
pub mod kalamine;
pub mod keystrokes;

use std::collections::HashMap;

use keystrokes::Keystrokes;

use crate::hands::Finger;
use crate::kalamine::{Corpus, Layout};

fn normalize<K>(map: &mut HashMap<K, f32>) {
    let total: f32 = map.values().sum();
    map.values_mut().for_each(|x| *x /= total);
}

// fn sort_vec_by_value<K, V: PartialOrd>(vec: &mut Vec<(K, V)>) {
//     vec.sort_by(|(_, val1), (_, val2)| val2.partial_cmp(val1).unwrap());
// }

fn sort_vec_by_key<K: PartialOrd, V>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(key1, _), (key2, _)| key1.partial_cmp(key2).unwrap());
}

fn calc_finger_freq(
    sym_to_keystrokes: &HashMap<char, Keystrokes>,
    sym_freq: &HashMap<char, f32>,
) -> Vec<(Finger, f32)> {
    let mut finger_freq: HashMap<Finger, f32> = HashMap::new();

    for (symbol, freq) in sym_freq.iter() {
        let keystrokes = match sym_to_keystrokes.get(symbol) {
            Some(ks) => ks,
            None => continue,
        };
        for keycode in keystrokes {
            let finger = Finger::from(*keycode);
            finger_freq
                .entry(finger)
                .and_modify(|f| *f += freq)
                .or_insert(*freq);
        }
    }
    normalize(&mut finger_freq);
    let mut finger_freq: Vec<(_, _)> = finger_freq.into_iter().collect();
    sort_vec_by_key(&mut finger_freq);
    finger_freq
}

pub fn analyse(layout: &Layout, corpus: &Corpus) {
    let sym_to_keystrokes = keystrokes::build_keystrokes_map(layout);
    let stats = calc_finger_freq(&sym_to_keystrokes, &corpus.symbols);
    dbg!(&stats);
}

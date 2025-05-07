pub mod hands;
pub mod kalamine;
pub mod keystrokes;

use std::collections::HashMap;

use kalamine::PhysicalKey;
use keystrokes::Keystrokes;

use crate::hands::Finger;
use crate::kalamine::{Corpus, Layout};

fn normalize<K>(map: &mut HashMap<K, f32>) {
    let total: f32 = map.values().sum();
    map.values_mut().for_each(|x| *x /= total);
}

fn sort_vec_by_value<K, V: PartialOrd>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(_, val1), (_, val2)| val2.partial_cmp(val1).unwrap());
}

fn sort_vec_by_key<K: PartialOrd, V>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(key1, _), (key2, _)| key1.partial_cmp(key2).unwrap());
}

fn map_to_vec<K, V>(map: HashMap<K, V>) -> Vec<(K, V)> {
    map.into_iter().collect()
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

fn calc_sfb(
    sym_to_keystrokes: &HashMap<char, Keystrokes>,
    bigrams_freq: &HashMap<[char; 2], f32>,
) -> HashMap<[char; 2], f32> {
    let mut sfb: HashMap<[char; 2], f32> = HashMap::new();

    for (&bigram, &freq) in bigrams_freq {
        if &bigram[0] == &bigram[1] {
            continue;
        }

        let ks1 = match sym_to_keystrokes.get(&bigram[0]) {
            Some(ks) => ks,
            None => continue,
        };
        let ks2 = match sym_to_keystrokes.get(&bigram[1]) {
            Some(ks) => ks,
            None => continue,
        };

        let mut prev_finger: Option<Finger> = None;
        for &key in ks1.iter().chain(ks2.iter()) {
            let finger = Finger::from(key);
            if let Some(prev_finger) = prev_finger {
                if finger == prev_finger {
                    sfb.entry(bigram).and_modify(|f| *f += freq).or_insert(freq);
                    break;
                }
            }
            prev_finger = Some(finger);
        }
    }
    sfb
}

pub fn analyse(layout: &Layout, corpus: &Corpus) {
    let sym_to_keystrokes = keystrokes::build_keystrokes_map(layout);

    let stats = calc_finger_freq(&sym_to_keystrokes, &corpus.symbols);
    dbg!(&stats);

    let sfb = calc_sfb(&sym_to_keystrokes, &corpus.digrams);
    let mut sfb = map_to_vec(sfb);
    sort_vec_by_value(&mut sfb);
    sfb.reverse();
    dbg!(&sfb);
}

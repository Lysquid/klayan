use std::collections::HashMap;

use crate::hands::Finger;
use crate::keystrokes::Keystrokes;

pub fn calc_sfb(
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

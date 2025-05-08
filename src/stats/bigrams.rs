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
                    // TODO: SKU if same key
                    sfb.entry(bigram).and_modify(|f| *f += freq).or_insert(freq);
                    break;
                }
            }
            prev_finger = Some(finger);
        }
    }
    sfb
}

#[cfg(test)]
#[rustfmt::skip] 
mod tests {

    use super::*;
    use crate::kalamine::PhysicalKey::*;

    #[test]
    fn clac_sfb_simple() {
        let sym_to_ks = HashMap::from([
            ('e', vec![KeyE]),
            ('r', vec![KeyR]),
            ('d', vec![KeyD]),
        ]);
        let sfb = HashMap::from([
            (['e','d'], 1.0),
            (['d','e'], 2.0),
        ]);
        let not_sfb = HashMap::from([
            (['e','r'], 3.0),
            (['r','e'], 4.0),
            (['r','d'], 5.0),
            (['d','r'], 6.0),
        ]);
        let mut bigrams_freq = sfb.clone();
        bigrams_freq.extend(not_sfb);
        let result = calc_sfb(&sym_to_ks, &bigrams_freq);
        assert_eq!(result, sfb);
    }

    #[test]
    fn clac_sfb_deadkey() {
        let sym_to_ks = HashMap::from([
            ('d', vec![KeyD]),
            ('é', vec![Quote, KeyE]),
        ]);
        let sfb = HashMap::from([
            (['é','d'], 1.0),
        ]);
        let not_sfb = HashMap::from([
            (['d','é'], 1.0),
            (['é','é'], 1.0),
        ]);
        let mut bigrams_freq = sfb.clone();
        bigrams_freq.extend(not_sfb);
        let result = calc_sfb(&sym_to_ks, &bigrams_freq);
        assert_eq!(result, sfb);
    }

}

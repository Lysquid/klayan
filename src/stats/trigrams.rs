use super::utils;
use crate::{
    hands::{Finger, RollDirection},
    kalamine::{PhysicalKey, Symbol},
    keyseq::KeySymbol,
};

type Trigram = [Symbol; 3];

pub fn trigram_stats(trigrams_freq: &Vec<([KeySymbol; 3], f32)>) -> TrigramStats {
    let mut sks: Vec<(Trigram, f32)> = Vec::new();
    let mut sfs: Vec<(Trigram, f32)> = Vec::new();
    let mut redirects: Vec<(Trigram, f32)> = Vec::new();
    let mut bad_redirects: Vec<(Trigram, f32)> = Vec::new();

    for (trigram_keys, freq) in trigrams_freq {
        let trigram = [
            trigram_keys[0].symbol(),
            trigram_keys[1].symbol(),
            trigram_keys[2].symbol(),
        ];
        let freq = *freq;
        let key1 = trigram_keys[0].key;
        let key2 = trigram_keys[1].key;
        let key3 = trigram_keys[2].key;

        if is_sks(key1, key2, key3) {
            sks.push((trigram, freq));
        } else if is_sfs(key1, key2, key3) {
            sfs.push((trigram, freq));
        }
        if is_redirect(key1, key2, key3) {
            if is_redirect_bad(key1, key2, key3) {
                bad_redirects.push((trigram, freq));
            } else {
                redirects.push((trigram, freq));
            }
        }
    }

    TrigramStats {
        total_sks: utils::result_sum(&sks),
        total_sfs: utils::result_sum(&sfs),
        total_redirects: utils::result_sum(&redirects),
        total_bad_redirects: utils::result_sum(&bad_redirects),
        list_sks: utils::result_vec(sks),
        list_sfs: utils::result_vec(sfs),
        list_redirects: utils::result_vec(redirects),
        list_bad_redirects: utils::result_vec(bad_redirects),
    }
}

pub fn is_sks(key1: PhysicalKey, _: PhysicalKey, key3: PhysicalKey) -> bool {
    key1 == key3
}

pub fn is_sfs(key1: PhysicalKey, _: PhysicalKey, key3: PhysicalKey) -> bool {
    key1.finger() == key3.finger()
}

pub fn is_redirect(key1: PhysicalKey, key2: PhysicalKey, key3: PhysicalKey) -> bool {
    let roll1 = key1.finger().roll_direction(key2.finger());
    let roll2 = key2.finger().roll_direction(key3.finger());
    match (roll1, roll2) {
        (RollDirection::Inside, RollDirection::Outside) => true,
        (RollDirection::Outside, RollDirection::Inside) => true,
        (_, _) => false,
    }
}

/// This function assumes the keys correspond to a redirect,
/// and just checks if it is a *bad* redirect
pub fn is_redirect_bad(key1: PhysicalKey, key2: PhysicalKey, key3: PhysicalKey) -> bool {
    is_bad_finger(key1.finger()) && is_bad_finger(key2.finger()) && is_bad_finger(key3.finger())
}

fn is_bad_finger(finger: Finger) -> bool {
    match finger {
        Finger::LeftIndex | Finger::RightIndex | Finger::Thumb => false,
        _ => true,
    }
}

pub struct TrigramStats {
    pub total_sks: f32,
    pub total_sfs: f32,
    pub total_redirects: f32,
    pub total_bad_redirects: f32,
    pub list_sks: Vec<(Trigram, f32)>,
    pub list_sfs: Vec<(Trigram, f32)>,
    pub list_redirects: Vec<(Trigram, f32)>,
    pub list_bad_redirects: Vec<(Trigram, f32)>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use PhysicalKey::*;

    #[test]
    fn redirect() {
        assert!(is_redirect(KeyJ, KeyL, KeyK));
        assert!(is_redirect(KeyK, KeyL, KeyJ));
        assert!(!is_redirect(KeyJ, KeyK, KeyL));
        assert!(!is_redirect(KeyK, KeyH, KeyJ)); // same finger
        assert!(!is_redirect(KeyL, KeyG, KeyK)); // different hands
        assert!(!is_redirect(KeyL, Space, Semicolon)); // space
    }

    #[test]
    fn bad_redirect() {
        assert!(!is_redirect_bad(KeyJ, KeyL, KeyK));
        assert!(is_redirect_bad(KeyK, Semicolon, KeyL));
    }
}

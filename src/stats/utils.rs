use std::collections::hash_map::Entry;
use std::collections::HashMap;

use itertools::Itertools;

use crate::kalamine::PhysicalKey;
use crate::keystrokes::Keystrokes;

pub fn normalize<K>(map: &mut HashMap<K, f32>) {
    let total: f32 = map.values().sum();
    map.values_mut().for_each(|x| *x /= total);
}

pub fn sort_vec_by_value<K, V: PartialOrd>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(_, val1), (_, val2)| val2.partial_cmp(val1).unwrap());
}

pub fn sort_vec_by_key<K: PartialOrd, V>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(key1, _), (key2, _)| key1.partial_cmp(key2).unwrap());
}

pub fn map_to_vec<K, V>(map: HashMap<K, V>) -> Vec<(K, V)> {
    map.into_iter().collect()
}

pub fn add_or_insert<K>(entry: Entry<'_, K, f32>, freq: f32) {
    entry.and_modify(|f| *f += freq).or_insert(freq);
}

pub fn bigram_two_keys_iter(
    sym_to_keystrokes: &HashMap<char, Keystrokes>,
    bigram: [char; 2],
) -> Option<impl Iterator<Item = (&PhysicalKey, &PhysicalKey)>> {
    let ks1 = sym_to_keystrokes.get(&bigram[0])?;
    let ks2 = sym_to_keystrokes.get(&bigram[1])?;
    Some(ks1.iter().chain(ks2.iter()).tuple_windows())
}

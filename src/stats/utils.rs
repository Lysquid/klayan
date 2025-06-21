use std::collections::hash_map::Entry;
#[cfg(test)]
use std::collections::hash_map::HashMap;

pub fn add_or_insert<K>(entry: Entry<'_, K, f32>, freq: f32) {
    entry.and_modify(|f| *f += freq).or_insert(freq);
}

pub fn result_sum<K>(vec: &Vec<(K, f32)>) -> f32 {
    vec.iter().map(|(_, v)| *v).sum::<f32>().abs()
}

pub fn result_vec<K: Clone>(mut vec: Vec<(K, f32)>) -> Vec<(K, f32)> {
    vec.sort_by(|(_, v1), (_, v2)| v2.partial_cmp(v1).unwrap());
    vec
}

#[cfg(test)]
pub fn round_result_map<K: Clone + Eq + std::hash::Hash>(map: HashMap<K, f32>) -> HashMap<K, f32> {
    map.iter().map(|(k, v)| (k.clone(), v.round())).collect()
}

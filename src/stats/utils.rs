use std::collections::hash_map::Entry;
use std::collections::HashMap;

// pub fn normalize<K>(map: &mut HashMap<K, f32>) {
//     let total: f32 = map.values().sum();
//     map.values_mut().for_each(|x| *x /= total);
// }

pub fn sort_vec_by_key<K: PartialOrd, V>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(key1, _), (key2, _)| key1.partial_cmp(key2).unwrap());
}

pub fn map_to_vec<K, V>(map: HashMap<K, V>) -> Vec<(K, V)> {
    map.into_iter().collect()
}

pub fn add_or_insert<K>(entry: Entry<'_, K, f32>, freq: f32) {
    entry.and_modify(|f| *f += freq).or_insert(freq);
}

pub fn result_sum<K>(vec: &Vec<(K, f32)>) -> f32 {
    vec.iter().map(|(_, v)| *v).sum::<f32>() * 100.0
}

fn vec_percentages<K: Clone>(vec: Vec<(K, f32)>) -> Vec<(K, f32)> {
    vec.iter().map(|(k, v)| (k.clone(), v * 100.0)).collect()
}

pub fn result_vec<K: Clone>(mut vec: Vec<(K, f32)>) -> Vec<(K, f32)> {
    vec.sort_by(|(_, v1), (_, v2)| v2.partial_cmp(v1).unwrap());
    vec_percentages(vec)
}

pub fn result_vec_from_map<K: Clone>(map: HashMap<K, f32>) -> Vec<(K, f32)> {
    result_vec(map.into_iter().collect())
}

pub fn result_vec_sorted_by_key<K: Clone + PartialOrd>(mut vec: Vec<(K, f32)>) -> Vec<(K, f32)> {
    vec.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());
    vec_percentages(vec)
}

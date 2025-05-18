use std::collections::hash_map::Entry;

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

pub fn result_vec_sorted_by_key<K: Clone + PartialOrd>(mut vec: Vec<(K, f32)>) -> Vec<(K, f32)> {
    vec.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());
    vec_percentages(vec)
}

#[cfg(test)]
pub fn round_result_vec<K: Clone>(vec: Vec<(K, f32)>) -> Vec<(K, f32)> {
    vec.iter().map(|(k, v)| (k.clone(), v.round())).collect()
}

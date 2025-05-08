use std::collections::HashMap;

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

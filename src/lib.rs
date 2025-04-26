use std::collections::HashMap;

use layout::{Key, Finger};
use serde_json::Value;
mod layout;
mod corups;

fn normalize<K>(map: &mut HashMap<K, f64>) {
    let total: f64 = map.values().sum();
    map.values_mut().for_each(|x| *x /= total);
}

fn sort_vec_by_value<K, V: PartialOrd>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(_, val1), (_, val2)| {val2.partial_cmp(val1).unwrap()});
}

fn sort_vec_by_key<K: PartialOrd, V>(vec: &mut Vec<(K, V)>) {
    vec.sort_by(|(key1, _), (key2, _)| {key1.partial_cmp(key2).unwrap()});
}


fn finger_frequency(layout: &HashMap<char, Key>, symbols: &Value) -> Vec<(Finger, f64)> {

    let mut finger_freq = HashMap::new();
    for (symbol, freq) in symbols.as_object().unwrap() {
        let symbol = symbol.as_str().chars().next().unwrap();
        let freq = freq.as_f64().unwrap();
        let finger = match layout.get(&symbol) {
            Some(key) => key.finger.clone(),
            None => {
                continue;
            },
        };
        finger_freq.entry(finger).and_modify(|f| *f += freq).or_insert(freq);
    }
    
    normalize(&mut finger_freq);
    let mut finger_freq: Vec<(_, _)> = finger_freq.into_iter().collect();
    sort_vec_by_key(&mut finger_freq);
    finger_freq
}

pub fn analyse(layout: Value, corpus: Value) {
    let layout = Key::build_map(layout);
    // dbg!(&layout[&'D']);
    let stats = finger_frequency(&layout, &corpus[&"symbols"]);
    dbg!(&stats);

}

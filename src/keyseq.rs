use log::warn;
use std::{collections::HashMap, hash::Hash};

use crate::{
    geometry::Row,
    hands::Finger,
    kalamine::{DeadKey, Mod, ModMapping, PhysicalKey, Symbol},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeySymbol {
    pub name: char,
    pub key: PhysicalKey,
    pub dead_key: bool,
    pub modifier: Mod,
}

impl KeySymbol {
    pub fn new(symbol: Symbol, key: PhysicalKey, modifier: Mod) -> Self {
        Self {
            name: match symbol {
                Symbol::Character(c) => c,
                Symbol::DeadKey(c) => c,
            },
            key,
            dead_key: match symbol {
                Symbol::Character(_) => false,
                Symbol::DeadKey(_) => true,
            },
            modifier,
        }
    }

    pub fn symbol(&self) -> Symbol {
        match self.dead_key {
            false => Symbol::Character(self.name),
            true => Symbol::DeadKey(self.name),
        }
    }
}

pub fn build_keyseq_map(
    layout_keymap: &HashMap<PhysicalKey, ModMapping>,
    layout_deadkeys: &HashMap<DeadKey, HashMap<Symbol, Symbol>>,
) -> HashMap<char, Vec<KeySymbol>> {
    let mut base_keysym_map: HashMap<char, KeySymbol> = HashMap::new();
    let mut deadkeys_map: HashMap<DeadKey, Vec<KeySymbol>> = HashMap::new();

    // One key characters
    for (&physical_key, mod_mapping) in layout_keymap.iter() {
        for (modifier, symbol) in mod_mapping.map {
            if let Some(symbol) = symbol {
                let keysym = KeySymbol::new(symbol, physical_key, modifier);
                match symbol {
                    Symbol::Character(c) => {
                        if is_bettery_keysym(&keysym, base_keysym_map.get(&c)) {
                            base_keysym_map.insert(c, keysym);
                        }
                    }
                    Symbol::DeadKey(c) => {
                        deadkeys_map.insert(DeadKey { name: c }, vec![keysym]);
                    }
                }
            }
            // TODO: warn if incompatible key is used, such as IntlBackslash with ANSI
            // and don't add it to the map
        }
    }

    // Dead keys layers
    let mut dk_layer_to_parse: Vec<DeadKey> = layout_deadkeys.keys().cloned().collect();
    // character to key-symbols sequence map:
    let mut keyseq_map: HashMap<char, Vec<KeySymbol>> = HashMap::new();

    while let Some(deadkey) = dk_layer_to_parse.pop() {
        let layer = match layout_deadkeys.get(&deadkey) {
            Some(s) => s,
            None => {
                warn!("No layer defined for dead key '{deadkey}'");
                continue;
            }
        };
        let dk_keyseq = match deadkeys_map.get(&deadkey).cloned() {
            Some(ks_sequence) => ks_sequence,
            None => continue, // Dead key that we don't know how to trigger yet
        };

        for (trigger_sym, output_sym) in layer {
            // Get the key symbol of the trigger
            let trigger_keysym: &KeySymbol = match trigger_sym {
                Symbol::Character(c) => match base_keysym_map.get(&c) {
                    Some(ks) => ks,
                    None => {
                        warn!("Symbol '{c}' from dead key layer '{deadkey}' is not on the base layers");
                        continue;
                    }
                },
                Symbol::DeadKey(c) => {
                    // Double press the dead key (e.g. ** -> ¨ in Ergo-L)
                    if c == &deadkey.name {
                        dk_keyseq.last().unwrap()
                    } else {
                        warn!("Invalid trigger '{trigger_sym}' on dead key layer '{deadkey}'");
                        continue;
                    }
                }
            };

            // Build the key sequence to do the output symbol
            let mut keyseq = dk_keyseq.clone();

            match output_sym {
                Symbol::Character(c) => {
                    keyseq.push(trigger_keysym.clone());
                    if is_better_keyseq(&keyseq, keyseq_map.get(c)) {
                        keyseq_map.insert(*c, keyseq);
                    }
                }
                Symbol::DeadKey(c) => {
                    let dk = DeadKey { name: *c };
                    if !dk_layer_to_parse.contains(&dk) {
                        dk_layer_to_parse.push(dk);
                    }
                    let ks =
                        KeySymbol::new(*output_sym, trigger_keysym.key, trigger_keysym.modifier);
                    keyseq.push(ks);
                    if is_better_keyseq(&keyseq, deadkeys_map.get(&dk)) {
                        deadkeys_map.insert(dk, keyseq);
                    }
                }
            }
        }
    }

    keyseq_map.extend(base_keysym_map.into_iter().map(|(c, key)| (c, vec![key])));
    keyseq_map
}

fn is_bettery_keysym(keysym: &KeySymbol, old_keysym: Option<&KeySymbol>) -> bool {
    match old_keysym {
        None => true,
        Some(old_keysym) => {
            if keysym.modifier != old_keysym.modifier {
                keysym.modifier < old_keysym.modifier
            } else {
                let dfh = Row::distance(Row::Middle, keysym.key.row()); // distance from home
                let old_dfh = Row::distance(Row::Middle, old_keysym.key.row());
                if dfh != old_dfh {
                    dfh < old_dfh
                } else {
                    warn!("non-deterministic choice between two keys for the same character");
                    false
                }
            }
        }
    }
}

fn is_better_keyseq(ks: &Vec<KeySymbol>, old_ks: Option<&Vec<KeySymbol>>) -> bool {
    match old_ks {
        None => true,
        Some(old_ks) => {
            if ks.len() == old_ks.len() {
                // if same length, prefer the one using the least mods
                let mod_count = count_mods(ks);
                let old_mod_count = count_mods(old_ks);
                if mod_count != old_mod_count {
                    mod_count < old_mod_count
                } else {
                    // if same length and mods, prefer the one using the thumb
                    let thumb_count = count_thumbs(ks);
                    let old_thumb_count = count_thumbs(old_ks);
                    if thumb_count != old_thumb_count {
                        thumb_count > old_thumb_count
                    } else {
                        warn!("non-deterministic choice between two key sequences for the same character");
                        false
                    }
                }
            } else {
                ks.len() < old_ks.len()
            }
        }
    }
}

fn count_mods(sequence: &Vec<KeySymbol>) -> u32 {
    sequence
        .iter()
        .map(|keysym| keysym.modifier.mod_count())
        .sum()
}

fn count_thumbs(sequence: &Vec<KeySymbol>) -> usize {
    sequence
        .iter()
        .filter(|&x| x.key.finger() == Finger::Thumb)
        .count()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use Mod::*;
    use PhysicalKey::*;
    use Symbol::Character;

    #[test]
    fn test_build_keyseq_map() {
        let keymap = HashMap::from([
            (KeyA, ModMapping::from(vec!["a", "A", "(", ")"])),
            (KeyB, ModMapping::from(vec!["b", "b", "b", "b"])),
            (Digit5, ModMapping::from(vec!["b", "b", "b", "b"])),
            (KeyG, ModMapping::from(vec!["g"])),
            (Space, ModMapping::from(vec![" "])),
            (Quote, ModMapping::from(vec!["'"])),
            (Period, ModMapping::from(vec!["."])),
            (Minus, ModMapping::from(vec!["*^", "-"])),
        ]);
        let deadkeys = HashMap::from([
            (
                DeadKey { name: '^' },
                HashMap::from([
                    (Character('a'), Character('â')),
                    (Character('A'), Character('Â')),
                    (Character('('), Character('{')),
                    (Character(')'), Character('}')),
                    (Character('i'), Character('ï')),
                    (Character('.'), Character('.')),
                    (Character('\''), Character('’')),
                    (Character(' '), Character('’')),
                    (Character('g'), Symbol::DeadKey('µ')),
                    (Symbol::DeadKey('^'), Symbol::DeadKey('¨')),
                ]),
            ),
            (
                DeadKey { name: 'µ' },
                HashMap::from([
                    (Character('a'), Character('α')),
                    (Character('g'), Character('γ')),
                ]),
            ),
            (
                DeadKey { name: '¨' },
                HashMap::from([
                    (Character('a'), Character('ä')),
                    (Character('.'), Character('.')),
                ]),
            ),
        ]);
        let keystrokes_map = build_keyseq_map(&keymap, &deadkeys);

        let ks_a = KeySymbol::new(Character('a'), KeyA, Base);
        let ks_a_maj = KeySymbol::new(Character('A'), KeyA, Shift);
        let ks_lp = KeySymbol::new(Character('('), KeyA, Altgr);
        let ks_rp = KeySymbol::new(Character(')'), KeyA, AltgrShift);
        let ks_b = KeySymbol::new(Character('b'), KeyB, Base);
        let ks_g = KeySymbol::new(Character('g'), KeyG, Base);
        let ks_space = KeySymbol::new(Character(' '), Space, Base);
        let ks_quote = KeySymbol::new(Character('\''), Quote, Base);
        let ks_period = KeySymbol::new(Character('.'), Period, Base);
        let ks_mu = KeySymbol::new(Symbol::DeadKey('µ'), KeyG, Base);
        let ks_minus = KeySymbol::new(Character('-'), Minus, Shift);
        let ks_caret = KeySymbol::new(Symbol::DeadKey('^'), Minus, Base);
        let ks_diae = KeySymbol::new(Symbol::DeadKey('¨'), Minus, Base);
        let expected = HashMap::from([
            ('a', vec![ks_a.clone()]),
            ('A', vec![ks_a_maj.clone()]),
            ('(', vec![ks_lp.clone()]),
            (')', vec![ks_rp.clone()]),
            ('b', vec![ks_b.clone()]),
            ('g', vec![ks_g.clone()]),
            (' ', vec![ks_space.clone()]),
            ('\'', vec![ks_quote.clone()]),
            ('.', vec![ks_period.clone()]),
            ('-', vec![ks_minus.clone()]),
            ('â', vec![ks_caret.clone(), ks_a.clone()]),
            ('Â', vec![ks_caret.clone(), ks_a_maj.clone()]),
            ('{', vec![ks_caret.clone(), ks_lp.clone()]),
            ('}', vec![ks_caret.clone(), ks_rp.clone()]),
            ('’', vec![ks_caret.clone(), ks_space.clone()]),
            ('ä', vec![ks_caret.clone(), ks_diae.clone(), ks_a.clone()]),
            ('α', vec![ks_caret.clone(), ks_mu.clone(), ks_a.clone()]),
            ('γ', vec![ks_caret.clone(), ks_mu.clone(), ks_g.clone()]),
        ]);
        for (sym, expected_seq) in expected.iter() {
            // Simplify debug with one to one comparison
            assert!(
                keystrokes_map.contains_key(sym),
                "keystrokes_map does not contain symbol: {}",
                sym
            );
            assert_eq!(
                keystrokes_map.get(sym).unwrap(),
                expected_seq,
                "symbol: {}",
                sym
            );
        }
        assert_eq!(keystrokes_map, expected);
    }
}

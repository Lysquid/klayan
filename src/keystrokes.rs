use log::warn;
use std::{
    collections::HashMap,
    hash::Hash,
};

use crate::{
    hands::Finger,
    kalamine::{DeadKey, ModMapping, PhysicalKey, Symbol},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeySymbol {
    pub name: char,
    pub key: PhysicalKey,
    pub dead_key: bool,
}

impl KeySymbol {

    pub fn new(symbol: Symbol, key: PhysicalKey) -> Self {
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
    for (&physical_key, symbols) in layout_keymap.iter() {
        for symbol in [
            symbols.base,
            symbols.shift,
            symbols.altgr,
            symbols.altgr_shift,
        ] {
            if let Some(symbol) = symbol {
                let keysym = KeySymbol::new(symbol, physical_key);
                match symbol {
                    Symbol::Character(c) => {
                        base_keysym_map.insert(c, keysym);
                        // TODO: handle duplicates (take the one with least mods)
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
                    dk_layer_to_parse.push(dk);
                    let ks = KeySymbol::new(*output_sym, trigger_keysym.key);
                    keyseq.push(ks);
                    if is_better_keyseq(&keyseq, deadkeys_map.get(&dk)) {
                        deadkeys_map.insert(dk, keyseq);
                    }
                }
            }
        }
    }

    keyseq_map.extend(
        base_keysym_map
            .into_iter()
            .map(|(c, key)| (c, vec![key])),
    );
    keyseq_map
}

fn is_better_keyseq(ks: &Vec<KeySymbol>, old_ks: Option<&Vec<KeySymbol>>) -> bool {
    match old_ks {
        None => true,
        Some(old_ks) => {
            if ks.len() == old_ks.len() {
                // if same length, prefer the one using the thumb
                count_thumbs(ks) > count_thumbs(old_ks)
            } else {
                ks.len() < old_ks.len()
            }
        }
    }
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
    use PhysicalKey::*;
    use Symbol::Character;

    #[test]
    fn test_build_keyseq_map() {
        let keymap = HashMap::from([
            (KeyA, ModMapping::from(vec!["a", "A", "(", ")"])),
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

        let ks_a = KeySymbol::new(Character('a'), KeyA);
        let ks_a_maj = KeySymbol::new(Character('A'), KeyA);
        let ks_lp = KeySymbol::new(Character('('), KeyA);
        let ks_rp = KeySymbol::new(Character(')'), KeyA);
        let ks_g = KeySymbol::new(Character('g'), KeyG);
        let ks_space = KeySymbol::new(Character(' '), Space);
        let ks_quote = KeySymbol::new(Character('\''), Quote);
        let ks_period = KeySymbol::new(Character('.'), Period);
        let ks_mu = KeySymbol::new(Symbol::DeadKey('µ'), KeyG);
        let ks_minus = KeySymbol::new(Character('-'), Minus);
        let ks_caret = KeySymbol::new(Symbol::DeadKey('^'), Minus);
        let ks_diae = KeySymbol::new(Symbol::DeadKey('¨'), Minus);
        let expected = HashMap::from([
            ('a', vec![ks_a.clone()]),
            ('A', vec![ks_a_maj.clone()]),
            ('(', vec![ks_lp.clone()]),
            (')', vec![ks_rp.clone()]),
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

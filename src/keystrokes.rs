use log::warn;
use std::collections::HashMap;

use crate::kalamine::{DeadKey, Layout, ModMapping, PhysicalKey, Symbol};

pub type Keystrokes = Vec<PhysicalKey>;

pub fn build_keystrokes_map(layout: &Layout) -> HashMap<char, Keystrokes> {
    build_keystrokes_map_internal(&layout.keymap, &layout.deadkeys)
}

fn build_keystrokes_map_internal(
    layout_keymap: &HashMap<PhysicalKey, ModMapping>,
    layout_deadkeys: &HashMap<DeadKey, HashMap<Symbol, Symbol>>,
) -> HashMap<char, Keystrokes> {
    // TODO: split this method into 2
    // - validation and conversion to a new layout struct
    //   (this data structure would only let express valid layouts,
    //   by using value semantics (Deadkeys contain there own map of PhysicalKey -> char))
    // - the rest stay in this method, the logic to convert the new
    //   layout struct to the keystrokes hashmap
    // Rational:
    // - less complicated functions, easier to read and less error prone
    // - less None handling in this function with the right data structure
    // - separation of concerns (right now validation and conversion are mixed up)
    // - error callback in validation to test the invalid JSON layout
    // - it's unlikely, but maybe someone would want to reuse the validated layout
    //   (exposed as another lib)
    // When to do this refactor: maybe after writing a few analysis functions,
    // to see if it would be annoying to keep the mod information in the new layout struct
    //
    // EDIT: actually YAGNI
    // If I need in the end, it's better to do it later so I don't refactor it multiple times

    let mut base_map: HashMap<char, PhysicalKey> = HashMap::new();
    let mut deadkeys_map: HashMap<DeadKey, Keystrokes> = HashMap::new();

    // One key characters
    for (physical_key, symbols) in layout_keymap.iter() {
        for symbol in [
            symbols.base,
            symbols.shift,
            symbols.altgr,
            symbols.altgr_shift,
        ] {
            if let Some(symbol) = symbol {
                match symbol {
                    Symbol::Character(c) => {
                        base_map.insert(c, *physical_key);
                        // TODO: handle duplicates (take the shortest)
                    }
                    Symbol::DeadKey(c) => {
                        deadkeys_map.insert(DeadKey { name: c }, vec![*physical_key]);
                    }
                }
            }
        }
    }

    // Dead keys
    let mut dk_layer_to_parse: Vec<DeadKey> = layout_deadkeys.keys().cloned().collect();
    let mut map: HashMap<char, Keystrokes> = HashMap::new();

    while let Some(deadkey) = dk_layer_to_parse.pop() {
        let symbols = match layout_deadkeys.get(&deadkey) {
            Some(s) => s,
            None => {
                warn!("No layer defined for dead key '{deadkey}'");
                continue;
            }
        };
        let dk_keystrokes = match deadkeys_map.get(&deadkey).cloned() {
            Some(ks) => ks,
            None => continue, // Dead key that we don't know how to trigger yet
        };

        for (trigger, symbol) in symbols {
            // Build the sequence of keystrokes to input this symbol
            let mut ks = dk_keystrokes.clone();
            match trigger {
                Symbol::Character(c) => {
                    ks.push(match base_map.get(c) {
                        Some(key) => *key,
                        None => {
                            warn!("Symbol '{c}' from dead key layer '{deadkey}' is not on the base layers");
                            continue;
                        },
                    });
                }
                Symbol::DeadKey(c) => {
                    if c == &deadkey.name {
                        ks.push(*ks.last().unwrap());
                    } else {
                        warn!("Invalid trigger '{trigger}' on dead key layer '{deadkey}'");
                        continue;
                    }
                }
            };

            match symbol {
                Symbol::Character(c) => {
                    map.insert(*c, ks);
                }
                Symbol::DeadKey(c) => {
                    let dk = DeadKey { name: *c };
                    dk_layer_to_parse.push(dk);
                    deadkeys_map.insert(dk, ks);
                }
            }
        }
    }

    map.extend(base_map.into_iter().map(|(c, key)| (c, vec![key])));
    return map;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use PhysicalKey::*;
    use Symbol::Character;

    #[test]
    fn build_keystrokes_map() {
        let keymap = HashMap::from([
            (KeyA, ModMapping::from(vec!["a", "A", "(", ")"])),
            (KeyG, ModMapping::from(vec!["g"])),
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
                    (Character('g'), Symbol::DeadKey('µ')),
                ]),
            ),
            (
                DeadKey { name: 'µ' },
                HashMap::from([
                    (Character('a'), Character('α')),
                    (Character('g'), Character('γ')),
                ]),
            ),
        ]);
        let keystrokes_map = build_keystrokes_map_internal(&keymap, &deadkeys);
        let expected = HashMap::from([
            ('a', vec![KeyA]),
            ('A', vec![KeyA]),
            ('(', vec![KeyA]),
            (')', vec![KeyA]),
            ('g', vec![KeyG]),
            ('-', vec![Minus]),
            ('â', vec![Minus, KeyA]),
            ('Â', vec![Minus, KeyA]),
            ('{', vec![Minus, KeyA]),
            ('}', vec![Minus, KeyA]),
            ('α', vec![Minus, KeyG, KeyA]),
            ('γ', vec![Minus, KeyG, KeyG]),
        ]);
        assert_eq!(keystrokes_map, expected);
    }
}

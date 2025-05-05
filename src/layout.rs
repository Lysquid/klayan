use crate::symbols::{DeadKey, ModMapping, Symbol};
use log::warn;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]
pub enum PhysicalKey {
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    Semicolon,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Period,
    Slash,
    Space,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Digit0,
    Minus,
    Equal,
    BracketLeft,
    BracketRight,
    Quote,
    Backquote,
    Backslash,
    IntlBackslash,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Geometry {
    Ergo,
    ISO,
    Compact,
}

#[derive(Debug, serde::Deserialize)]
pub struct Layout {
    // name: String,
    // description: String,
    // geometry: Geometry,
    keymap: HashMap<PhysicalKey, ModMapping>,
    deadkeys: HashMap<DeadKey, HashMap<Symbol, Symbol>>,
    // altgr: bool,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    Thumb,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

impl Finger {
    pub fn from(input: PhysicalKey) -> Self {
        use PhysicalKey::*;
        match input {
            Space => Self::Thumb,
            Digit1 | KeyQ | KeyA | KeyZ | IntlBackslash => Self::LeftPinky,
            Digit2 | KeyW | KeyS | KeyX => Self::LeftRing,
            Digit3 | KeyE | KeyD | KeyC => Self::LeftMiddle,
            Digit4 | KeyR | KeyF | KeyV | Digit5 | KeyT | KeyG | KeyB => Self::LeftIndex,
            Digit6 | KeyY | KeyH | KeyN | Digit7 | KeyU | KeyJ | KeyM => Self::RightIndex,
            Digit8 | KeyI | KeyK | Comma => Self::RightMiddle,
            Digit9 | KeyO | KeyL | Period => Self::RightRing,
            Digit0 | KeyP | Semicolon | Slash | Minus | Equal | BracketLeft | BracketRight
            | Quote | Backquote | Backslash => Self::RightPinky,
        }
    }
}

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
mod test {
    use std::collections::HashMap;

    use super::PhysicalKey::*;
    use super::Symbol::Character;
    use super::*;

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

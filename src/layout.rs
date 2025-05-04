use std::collections::HashMap;

use serde::Deserializer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]
pub enum Keycode {
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

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Symbol(char),
    DeadKey(char),
    None,
}

impl<'de> serde::Deserialize<'de> for Symbol {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let mut iter = s.chars();
        let first_char = match iter.next() {
            Some(c) => c,
            None => return Ok(Symbol::None),
        };
        let second_char = iter.next();
        if let Some(_) = iter.next() {
            return Err(serde::de::Error::custom(
                "Unexpected extra characters in input",
            ));
        }

        match second_char {
            Some(c) => {
                if first_char == '*' {
                    Ok(Symbol::DeadKey(c))
                } else {
                    Err(serde::de::Error::custom(
                        "Unexpected extra character in input",
                    ))
                }
            }
            None => Ok(Symbol::Symbol(first_char)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_symbol_single_character() {
        let json = r#""a""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::Symbol('a'));
    }

    #[test]
    fn deserialize_symbol_dead_key() {
        let json = r#""**""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::DeadKey('*'));
    }

    #[test]
    fn deserialize_symbol_none() {
        let json = r#""""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::None);
    }

    #[test]
    fn deserialize_symbol_invalid_extra_characters() {
        let json = r#""*ab""#;
        let result: Result<Symbol, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_symbol_invalid_dead_key_format() {
        let json = r#""ab""#;
        let result: Result<Symbol, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Layout {
    name: String,
    description: String,
    geometry: Geometry,
    keymap: HashMap<Keycode, Vec<Symbol>>,
    deadkeys: HashMap<String, HashMap<String, String>>,
    altgr: bool,
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
    pub fn from(input: Keycode) -> Self {
        use Keycode::*;
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

pub type Keystrokes = Vec<Keycode>;

pub fn build_sym_to_keystrokes_map(layout: &Layout) -> HashMap<char, Keystrokes> {
    let mut map: HashMap<char, Keystrokes> = HashMap::new();

    // One key characters
    for (keycode, symbols) in layout.keymap.iter() {
        for symbol in symbols {
            match symbol {
                Symbol::Symbol(c) => {
                    map.insert(*c, vec![*keycode]);
                }
                // TODO: handle dead keys
                Symbol::DeadKey(c) => continue,
                Symbol::None => continue,
            }
        }
    }
    // TODO: Dead key characters
    return map;
}

use std::collections::HashMap;

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

#[derive(Debug, serde::Deserialize)]
pub struct Layout {
    name: String,
    description: String,
    geometry: Geometry,
    keymap: HashMap<Keycode, Vec<String>>,
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
            Digit2 | KeyW | KeyS | KeyX   => Self::LeftRing,
            Digit3 | KeyE | KeyD | KeyC   => Self::LeftMiddle,
            Digit4 | KeyR | KeyF | KeyV |
            Digit5 | KeyT | KeyG | KeyB   => Self::LeftIndex,
            Digit6 | KeyY | KeyH | KeyN |
            Digit7 | KeyU | KeyJ | KeyM   => Self::RightIndex,
            Digit8 | KeyI | KeyK | Comma  => Self::RightMiddle,
            Digit9 | KeyO | KeyL | Period => Self::RightRing,
            Digit0 | KeyP | Semicolon | Slash |
            Minus | Equal | BracketLeft | BracketRight |
            Quote | Backquote | Backslash => Self::RightPinky
        }
    }
    
}


pub type Keystrokes = Vec<Keycode>;

pub fn build_sym_to_keystrokes_map(layout: &Layout) -> HashMap<char, Keystrokes> {
    let mut map: HashMap<char, Keystrokes> = HashMap::new();
    
    // One key characters
    for (keycode, symbols) in layout.keymap.iter() {

        for symbol in symbols {
            // TODO: handle dead keys
            let symbol = symbol.chars().next().unwrap();
            if let Some(keystrokes) = map.get_mut(&symbol) {
                // TODO: edge case, if multiple keep the shortest
            } else {
                map.insert(symbol, vec![*keycode]);
            }
        }
    }
    // TODO: Dead key characters
    return map;
}

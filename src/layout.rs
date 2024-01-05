use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    
    fn from_keycode(keycode: &str) -> Option<Self> {
        match keycode {
            "Space" => Some(Self::Thumb),
            "Digit1" | "KeyQ" | "KeyA" | "KeyZ" | "IntlBackslash" => Some(Self::LeftPinky),
            "Digit2" | "KeyW" | "KeyS" | "KeyX"   => Some(Self::LeftRing),
            "Digit3" | "KeyE" | "KeyD" | "KeyC"   => Some(Self::LeftMiddle),
            "Digit4" | "KeyR" | "KeyF" | "KeyV" |
            "Digit5" | "KeyT" | "KeyG" | "KeyB"   => Some(Self::LeftIndex),
            "Digit6" | "KeyY" | "KeyH" | "KeyN" |
            "Digit7" | "KeyU" | "KeyJ" | "KeyM"   => Some(Self::RightIndex),
            "Digit8" | "KeyI" | "KeyK" | "Comma"  => Some(Self::RightMiddle),
            "Digit9" | "KeyO" | "KeyL" | "Period" => Some(Self::RightRing),
            "Digit0" | "KeyP" | "Semicolon" | "Slash" |
            "Minus" | "Equal" | "BracketLeft" | "BracketRight" |
            "Quote" | "Backquote" | "Backslash"   => Some(Self::RightPinky),
            _ => None,
        }        
    }
}

#[derive(Debug)]
pub struct Key {
    pub finger: Finger,
}

impl Key {

    pub fn build_map(layout: Value) -> HashMap<char, Key> {
        let mut map = HashMap::new();
        for (keycode, symbols) in layout["keymap"].as_object().unwrap() {
            for symbol in symbols.as_array().unwrap() {
                let symbol = symbol.as_str().unwrap().chars().next().unwrap();
                let key = Key { finger: Finger::from_keycode(keycode).unwrap() };
                map.insert(symbol, key);
            }
        }
        map
    }
}
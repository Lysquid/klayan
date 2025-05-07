use crate::kalamine::symbols::{DeadKey, ModMapping, Symbol};
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
    pub keymap: HashMap<PhysicalKey, ModMapping>,
    pub deadkeys: HashMap<DeadKey, HashMap<Symbol, Symbol>>,
    // altgr: bool,
}

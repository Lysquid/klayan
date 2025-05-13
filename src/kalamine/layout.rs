use crate::{
    geometry::Row,
    hands::{Finger, Hand},
    kalamine::symbols::{DeadKey, ModMapping, Symbol},
};
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Layout {
    // name: String,
    // description: String,
    // geometry: Geometry,
    pub keymap: HashMap<PhysicalKey, ModMapping>,
    pub deadkeys: HashMap<DeadKey, HashMap<Symbol, Symbol>>,
    // altgr: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Geometry {
    Ergo,
    ISO,
    Compact,
}

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

impl PhysicalKey {
    pub fn finger(&self) -> Finger {
        // TODO: depend on geometry (opti) / angle mod
        use PhysicalKey::*;
        match self {
            Space => Finger::Thumb,
            Digit1 | KeyQ | KeyA | KeyZ | IntlBackslash => Finger::LeftPinky,
            Digit2 | KeyW | KeyS | KeyX => Finger::LeftRing,
            Digit3 | KeyE | KeyD | KeyC => Finger::LeftMiddle,
            Digit4 | KeyR | KeyF | KeyV | Digit5 | KeyT | KeyG | KeyB => Finger::LeftIndex,
            Digit6 | KeyY | KeyH | KeyN | Digit7 | KeyU | KeyJ | KeyM => Finger::RightIndex,
            Digit8 | KeyI | KeyK | Comma => Finger::RightMiddle,
            Digit9 | KeyO | KeyL | Period => Finger::RightRing,
            Digit0 | KeyP | Semicolon | Slash | Minus | Equal | BracketLeft | BracketRight
            | Quote | Backquote | Backslash => Finger::RightPinky,
        }
    }

    #[rustfmt::skip]
    pub fn row(&self) -> Row {
        use PhysicalKey::*;
        match self {
            Backquote | Digit1 | Digit2 | Digit3 | Digit4 | Digit5 | Digit6
            | Digit7 | Digit8 | Digit9 | Digit0 | Minus | Equal => Row::Digits,
            KeyQ | KeyW | KeyE | KeyR | KeyT | KeyY | KeyU | KeyI | KeyO
            | KeyP | BracketLeft | BracketRight | Backslash => Row::Upper,
            KeyA | KeyS | KeyD | KeyF | KeyG | KeyH | KeyJ
            | KeyK | KeyL | Semicolon | Quote => Row::Middle,
            IntlBackslash | KeyZ | KeyX | KeyC | KeyV | KeyB
            | KeyN | KeyM | Comma | Period | Slash => Row::Lower,
            Space => Row::Spacebar,
        }
    }

    pub fn hand(&self) -> Hand {
        self.finger().hand()
    }
}

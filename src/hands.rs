use crate::kalamine::PhysicalKey;

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

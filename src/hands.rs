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
    pub fn from(key: PhysicalKey) -> Self {
        use PhysicalKey::*;
        match key {
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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Hand {
    Left,
    Right,
    Thumbs,
}

impl Hand {
    fn from_finger(finger: Finger) -> Self {
        use Finger::*;
        match finger {
            LeftPinky | LeftRing | LeftMiddle | LeftIndex => Self::Left,
            RightIndex | RightMiddle | RightRing | RightPinky => Self::Right,
            Thumb => Self::Thumbs,
        }
    }

    pub fn from(key: PhysicalKey) -> Self {
        Self::from_finger(Finger::from(key))
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Row {
    Spacebar = 0,
    Lower = 1,
    Middle = 2,
    Upper = 3,
    Digits = 4,
}

impl Row {
    pub fn from(key: PhysicalKey) -> Self {
        use PhysicalKey::*;
        match key {
            Backquote | Digit1 | Digit2 | Digit3 | Digit4 | Digit5 | Digit6 | Digit7 | Digit8
            | Digit9 | Digit0 | Minus | Equal => Self::Digits,
            KeyQ | KeyW | KeyE | KeyR | KeyT | KeyY | KeyU | KeyI | KeyO | KeyP | BracketLeft
            | BracketRight | Backslash => Self::Upper,
            KeyA | KeyS | KeyD | KeyF | KeyG | KeyH | KeyJ | KeyK | KeyL | Semicolon | Quote => {
                Self::Middle
            }
            IntlBackslash | KeyZ | KeyX | KeyC | KeyV | KeyB | KeyN | KeyM | Comma | Period
            | Slash => Self::Lower,
            Space => Self::Spacebar,
        }
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

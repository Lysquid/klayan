use crate::kalamine::PhysicalKey;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Finger {
    Thumb = 0,
    LeftPinky = 1,
    LeftRing = 2,
    LeftMiddle = 3,
    LeftIndex = 4,
    RightIndex = 5,
    RightMiddle = 6,
    RightRing = 7,
    RightPinky = 8,
}

impl Finger {
    pub fn from(key: PhysicalKey) -> Self {
        // TODO: depend on geometry (opti) / angle mod
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

    pub fn distance(&self, other: &Self) -> Option<u32> {
        if Hand::from_finger(*self) != Hand::from_finger(*other)
            || *self == Self::Thumb
            || *other == Self::Thumb
        {
            return None;
        }
        return Some((*self as u32).abs_diff(*other as u32));
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

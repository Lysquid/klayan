use crate::kalamine::PhysicalKey;

/// Keyboard geometry.
/// Opti versions: optimized finger placement with angle mod,
/// and later shift on num layer.
#[derive(Debug, Clone, Copy)]
pub enum Geometry {
    ISO,
    ISOOpti,
    ANSI,
    ANSIOpti,
    Ortho,
}

impl std::str::FromStr for Geometry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Geometry::*;
        match s.to_lowercase().as_str() {
            "iso" => Ok(ISO),
            "iso-angle-mod" => Ok(ISOOpti),
            "ansi" => Ok(ANSI),
            "ansi-angle-mod" => Ok(ISOOpti),
            "ortho" => Ok(Ortho),
            _ => Err(format!("Invalid geometry: {}", s)),
        }
    }
}

// Key size unit.
// It's 4 to be able to divide by 4 for staggered layout without having to use floats
pub const U: u32 = 4;

impl Geometry {
    pub fn horizontal_distance(&self, key1: PhysicalKey, key2: PhysicalKey) -> Option<u32> {
        let offset1 = self.key_horizontal_position(key1)?;
        let offset2 = self.key_horizontal_position(key2)?;
        return Some(offset1.abs_diff(offset2));
    }

    fn key_horizontal_position(&self, key: PhysicalKey) -> Option<u32> {
        // TODO: is Option really needed here?
        use Geometry::*;
        use PhysicalKey::*;
        match self {
            Ortho => self.key_horizontal_position_ortho(key),
            ISO | ISOOpti | ANSI | ANSIOpti => {
                const TAB: u32 = U + U / 2; // 1.5U
                const CAPS: u32 = U + 3 * U / 4; // 1.75U
                const SHIFT: u32 = 2 * U + U / 4; // 2.25U
                match key {
                    Backslash => match self {
                        ISO | ISOOpti => Some(CAPS + 11 * U),
                        ANSI | ANSIOpti => Some(TAB + 12 * U),
                        _ => panic!(),
                    },
                    IntlBackslash => match self {
                        ISO | ISOOpti => Some(SHIFT),
                        ANSI | ANSIOpti => None,
                        _ => panic!(),
                    },
                    _ => Some(match key.row() {
                        Row::Upper => self.key_horizontal_position_ortho(key)? + TAB - U,
                        Row::Middle => self.key_horizontal_position_ortho(key)? + CAPS - U,
                        Row::Lower => self.key_horizontal_position_ortho(key)? + SHIFT - U,
                        _ => self.key_horizontal_position_ortho(key)?,
                    }),
                }
            }
        }
    }

    fn key_horizontal_position_ortho(&self, key: PhysicalKey) -> Option<u32> {
        use PhysicalKey::*;
        match key {
            Backquote => Some(0),
            Digit1 | KeyQ | KeyA | KeyZ => Some(U),
            Digit2 | KeyW | KeyS | KeyX => Some(2 * U),
            Digit3 | KeyE | KeyD | KeyC => Some(3 * U),
            Digit4 | KeyR | KeyF | KeyV => Some(4 * U),
            Digit5 | KeyT | KeyG | KeyB => Some(5 * U),
            Digit6 | KeyY | KeyH | KeyN => Some(6 * U),
            Digit7 | KeyU | KeyJ | KeyM => Some(7 * U),
            Digit8 | KeyI | KeyK | Comma => Some(8 * U),
            Digit9 | KeyO | KeyL | Period => Some(9 * U),
            Digit0 | KeyP | Semicolon | Slash => Some(10 * U),
            Minus | BracketLeft | Quote | Backslash => Some(11 * U),
            Equal | BracketRight => Some(12 * U),
            Space | IntlBackslash => None,
        }
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
    pub fn distance(r1: Row, r2: Row) -> u32 {
        (r1 as u32).abs_diff(r2 as u32)
    }
}

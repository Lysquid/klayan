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
    pub fn hand(&self) -> Hand {
        use Finger::*;
        match self {
            LeftPinky | LeftRing | LeftMiddle | LeftIndex => Hand::Left,
            RightIndex | RightMiddle | RightRing | RightPinky => Hand::Right,
            Thumb => Hand::Thumbs,
        }
    }

    pub fn distance(f1: Finger, f2: Finger) -> Option<u32> {
        if f1.hand() != f2.hand() {
            return None;
        }
        return Some((f1 as u32).abs_diff(f2 as u32));
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Hand {
    Left,
    Right,
    Thumbs,
}

#[cfg(test)]
mod tests {

    use super::*;
    use Finger::*;

    #[test]
    fn finger_distance() {
        assert!(Finger::distance(LeftMiddle, LeftIndex).unwrap() == 1);
        assert!(Finger::distance(RightIndex, RightPinky).unwrap() == 3);
        assert!(Finger::distance(LeftIndex, LeftIndex).unwrap() == 0);
        assert!(Finger::distance(LeftIndex, RightIndex).is_none());
        assert!(Finger::distance(LeftIndex, Thumb).is_none());
        assert!(Finger::distance(Thumb, Thumb).unwrap() == 0);
    }
}

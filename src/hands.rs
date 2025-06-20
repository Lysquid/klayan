use strum;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Finger {
    LeftPinky = 1,
    LeftRing = 2,
    LeftMiddle = 3,
    LeftIndex = 4,
    RightIndex = 5,
    RightMiddle = 6,
    RightRing = 7,
    RightPinky = 8,
    Thumb = 0,
}

#[derive(Debug, PartialEq)]
pub enum RollDirection {
    Inside,
    Outside,
    SameFinger,
    DifferentHands,
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

    pub fn roll_direction(&self, second_finger: Finger) -> RollDirection {
        if *self == second_finger {
            RollDirection::SameFinger
        } else if self.hand() != second_finger.hand() {
            RollDirection::DifferentHands
        } else if self.hand() == Hand::Left {
            if second_finger > *self {
                RollDirection::Inside
            } else {
                RollDirection::Outside
            }
        } else {
            if second_finger > *self {
                RollDirection::Outside
            } else {
                RollDirection::Inside
            }
        }
    }

    fn prefered_height(&self) -> u32 {
        use Finger::*;
        match self {
            Thumb => 0,
            LeftPinky | RightPinky => 1,
            LeftRing | RightRing => 3,
            LeftMiddle | RightMiddle => 4,
            LeftIndex | RightIndex => 2,
        }
    }

    pub fn prefers_being_higher(&self, other_finger: Finger) -> bool {
        self.prefered_height() > other_finger.prefered_height()
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, strum::EnumIter)]
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

    #[test]
    fn roll_direction() {
        use RollDirection::*;
        assert_eq!(LeftMiddle.roll_direction(LeftIndex), Inside);
        assert_eq!(LeftIndex.roll_direction(LeftMiddle), Outside);
        assert_eq!(LeftIndex.roll_direction(LeftIndex), SameFinger);
        assert_eq!(LeftIndex.roll_direction(RightIndex), DifferentHands);

        assert_eq!(RightMiddle.roll_direction(RightIndex), Inside);
        assert_eq!(RightIndex.roll_direction(RightMiddle), Outside);
    }
}

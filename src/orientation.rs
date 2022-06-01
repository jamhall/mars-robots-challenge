use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Self::North => write!(formatter, "N"),
            Self::East => write!(formatter, "E"),
            Self::South => write!(formatter, "S"),
            Self::West => write!(formatter, "W"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_turn_left() {
        assert_eq!(Orientation::North.turn_left(), Orientation::West);
        assert_eq!(Orientation::West.turn_left(), Orientation::South);
        assert_eq!(Orientation::South.turn_left(), Orientation::East);
        assert_eq!(Orientation::East.turn_left(), Orientation::North);
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(Orientation::North.turn_right(), Orientation::East);
        assert_eq!(Orientation::East.turn_right(), Orientation::South);
        assert_eq!(Orientation::South.turn_right(), Orientation::West);
        assert_eq!(Orientation::West.turn_right(), Orientation::North);
    }
}

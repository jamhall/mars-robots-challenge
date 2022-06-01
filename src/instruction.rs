use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Left,
    Right,
    Forward,
}

impl Display for Instruction {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Self::Left => write!(formatter, "Left"),
            Self::Right => write!(formatter, "Right"),
            Self::Forward => write!(formatter, "Forward"),
        }
    }
}

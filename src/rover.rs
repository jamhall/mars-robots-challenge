use std::fmt;

use crate::coordinate::Coordinate;
use crate::houston::Houston;
use crate::instruction::Instruction;
use crate::orientation::Orientation;

#[derive(Debug)]
pub struct Rover {
    position: Coordinate,
    orientation: Orientation,
    lost: bool,
}

impl Rover {
    pub fn new(position: Coordinate, orientation: Orientation) -> Self {
        Self {
            position,
            orientation,
            lost: false,
        }
    }

    pub fn turn_left(&mut self) -> &mut Self {
        self.orientation = self.orientation.turn_left();
        self
    }

    pub fn turn_right(&mut self) -> &mut Self {
        self.orientation = self.orientation.turn_right();
        self
    }

    pub fn forward(&mut self, context: &mut Houston) -> &mut Self {
        if !self.lost {
            let (x, y) = self.position.tuple();
            let (new_x, new_y) = match self.orientation {
                Orientation::North => (0, 1),
                Orientation::East => (1, 0),
                Orientation::South => (0, -1),
                Orientation::West => (-1, 0),
            };
            let position = Coordinate::new(x + new_x, y + new_y);
            if !context.has_scent(&position) {
                if context.is_inside(&position) {
                    self.position = position;
                } else {
                    self.lost = true;
                    // leave a scent so other robots don't suffer the same fate...
                    context.leave_scent(&position);
                }
            }
        }
        self
    }

    pub fn execute_instruction(&mut self, houston: &mut Houston, instruction: Instruction) -> &mut Rover {
        match instruction {
            Instruction::Left => self.turn_left(),
            Instruction::Right => self.turn_right(),
            Instruction::Forward => self.forward(houston),
        };
        self
    }

    #[cfg(test)]
    pub fn position(&self) -> Coordinate {
        self.position
    }

    #[cfg(test)]
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn is_lost(&self) -> bool {
        self.lost
    }
}

impl fmt::Display for Rover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.position.tuple();
        if self.is_lost() {
            write!(f, "{} {} {} LOST", x, y, self.orientation)
        } else {
            write!(f, "{} {} {}", x, y, self.orientation)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_turn_left() {
        let position = Coordinate::new(10, 5);
        let mut rover = Rover::new(position, Orientation::North);
        assert!(matches!(rover.turn_left().orientation(), Orientation::West));
        assert!(matches!(rover.turn_left().orientation(), Orientation::South));
        assert!(matches!(rover.turn_left().orientation(), Orientation::East));
    }

    #[test]
    fn test_turn_right() {
        let position = Coordinate::new(10, 5);
        let mut rover = Rover::new(position, Orientation::North);
        assert!(matches!(rover.turn_right().orientation(), Orientation::East));
        assert!(matches!(rover.turn_right().orientation(), Orientation::South));
        assert!(matches!(rover.turn_right().orientation(), Orientation::West));
    }

    #[test]
    fn test_forward() {
        let mut houston = Houston::new(Coordinate::new(10, 10));

        let position = Coordinate::new(10, 5);
        let mut rover = Rover::new(position, Orientation::North);

        assert_eq!(rover.forward(&mut houston).position(), Coordinate::new(10, 6));

        rover.turn_left();

        assert_eq!(rover.forward(&mut houston).position(), Coordinate::new(9, 6));

        rover.turn_right();

        assert_eq!(rover.forward(&mut houston).position(), Coordinate::new(9, 7));

        rover.turn_right();

        assert_eq!(rover.forward(&mut houston).position(), Coordinate::new(10, 7));

        rover.turn_right();

        assert_eq!(rover.forward(&mut houston).position(), Coordinate::new(10, 6));

        rover.turn_right();
        rover.turn_right();
        rover.turn_right();

        rover.forward(&mut houston);

        assert_eq!(rover.is_lost(), true);
    }
}

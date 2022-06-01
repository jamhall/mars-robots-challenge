use crate::bounding_box::BoundingBox;
use crate::coordinate::Coordinate;

#[derive(PartialEq, Debug)]
pub struct Houston {
    bounding_box: BoundingBox,
    scents: Vec<Coordinate>,
}

impl Houston {
    pub fn new(position: Coordinate) -> Self {
        Self {
            bounding_box: position.into(),
            scents: Vec::new(),
        }
    }

    pub fn is_inside(&self, coordinate: &Coordinate) -> bool {
        let (x, y) = coordinate.tuple();
        self.bounding_box.is_inside(x, y)
    }

    pub fn has_scent(&self, coordinate: &Coordinate) -> bool {
        self.scents
            .iter()
            .filter(|predicate| *predicate == coordinate)
            .count()
            > 0
    }

    pub fn leave_scent(&mut self, position: &Coordinate) {
        self.scents.push(*position);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_scent() {
        let position = Coordinate::new(10, 25);
        let mut houston = Houston::new(position);
        let scent = Coordinate::new(8, 20);
        houston.leave_scent(&scent);
        assert!(houston.has_scent(&scent));
    }
}

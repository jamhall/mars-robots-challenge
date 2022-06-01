use crate::coordinate::Coordinate;

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    max_x: i64,
    max_y: i64,
}

impl BoundingBox {
    pub fn new(max_x: i64, max_y: i64) -> Self {
        Self { max_x, max_y }
    }

    pub fn is_inside(&self, x: i64, y: i64) -> bool {
        x <= self.max_x && y <= self.max_y
    }
}

impl From<Coordinate> for BoundingBox {
    fn from(coordinate: Coordinate) -> Self {
        let (x, y) = coordinate.tuple();
        BoundingBox::new(x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inside() {
        let bounding_box = BoundingBox::new(15, 10);
        assert_eq!(bounding_box.is_inside(5, 5), true);
        assert_eq!(bounding_box.is_inside(15, 10), true);
        assert_eq!(bounding_box.is_inside(20, 10), false);
        assert_eq!(bounding_box.is_inside(25, 5), false);
        assert_eq!(bounding_box.is_inside(15, 15), false);
    }
}

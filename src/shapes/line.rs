use super::{BoundingBox, GetBounding, Location};


pub struct Line {
    pub start: Location,
    pub end:   Location,
}


impl Line {
    pub fn new(start: Location, end: Location) -> Line {
        Line {
            start,
            end,
        }
    }

}

impl GetBounding for Line {
    fn bounding_box(&self) -> BoundingBox {
        let x = vec![self.start.x, self.end.x];
        let y = vec![self.start.y, self.end.y];

        BoundingBox::from((&x, &y))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Ensure the bounding box of a line holds expected values
    #[test]
    fn bounding_box() {
        // Note: test assumes start_x < end_x and start_y < end_y
        // If these values change such that that isn't true, it will break the test
        let start_x = 1.0;
        let start_y = 2.0;
        let end_x = 3.0;
        let end_y = 4.0;
        let line = Line::new(
            Location::new(start_x, start_y),
            Location::new(end_x, end_y),
        );

        let bounding_box = line.bounding_box();

        assert_eq!(bounding_box.bottom(), start_y);
        assert_eq!(bounding_box.top(), end_y);
        assert_eq!(bounding_box.left(), start_x);
        assert_eq!(bounding_box.right(), end_x);

        let top_left = bounding_box.top_left;

        assert_eq!(top_left.y, end_y);
        assert_eq!(top_left.x, start_x);

        let bottom_right = bounding_box.bottom_right;

        assert_eq!(bottom_right.x, end_x);
        assert_eq!(bottom_right.y, start_y);
    }
}
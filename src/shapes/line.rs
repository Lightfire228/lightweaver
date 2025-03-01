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

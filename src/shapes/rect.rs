use super::{BoundingBox, Dimensions, GetBounding, Location};


pub struct Rect {
    pub center: Location,
    pub dim:    Dimensions,
}

impl Rect {
    pub fn new(center: Location, dim: Dimensions) -> Rect {
        Rect {
            center,
            dim,
        }
    }

}

impl From<&BoundingBox> for Rect {
    fn from(bounding_box: &BoundingBox) -> Rect {

        let dim = Dimensions::new(
            bounding_box.right() - bounding_box.left(),
            bounding_box.top()   - bounding_box.bottom(),
        );

        let center = Location::new(
            (dim.width  / 2.0) + bounding_box.left(),
            (dim.height / 2.0) + bounding_box.bottom(),

        );

        Rect::new(
            center,
            dim,
        )
    }
}

impl GetBounding for Rect {
    fn bounding_box(&self) -> BoundingBox {

        let half_width  = &self.dim.width  / 2.0;
        let half_height = &self.dim.height / 2.0;

        BoundingBox {
            top_left: Location::new(
                &self.center.x - half_width,
                &self.center.y + half_height,
            ),
            bottom_right: Location::new(
                &self.center.x + half_width,
                &self.center.y - half_height,
            )
        }
    }
}

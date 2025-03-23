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


#[cfg(test)]
mod tests {
    use super::*;

    // Ensure the bounding box of a rect holds expected values
    #[test]
    fn bounding_box() {
        let top    = 5.0;
        let bottom = -2.0;
        let left   = 1.0;
        let right  = 3.5;

        let center_x = (left  + right)  / 2.0;
        let center_y = (top   + bottom) / 2.0;
        let width    =  right - left;
        let height   =  top   - bottom;

        let rect = Rect::new(
            Location  ::new(center_x, center_y),
            Dimensions::new(width,    height)
        );
        let bounding_box = rect.bounding_box();

        // Note: float compare, may not yield stable results
        assert_eq!(bounding_box.bottom(), bottom);
        assert_eq!(bounding_box.top(),    top);
        assert_eq!(bounding_box.left(),   left);
        assert_eq!(bounding_box.right(),  right);

        let top_left = bounding_box.top_left;

        assert_eq!(top_left.y, top);
        assert_eq!(top_left.x, left);

        let bottom_right = bounding_box.bottom_right;

        assert_eq!(bottom_right.y, bottom);
        assert_eq!(bottom_right.x, right);
    }
}

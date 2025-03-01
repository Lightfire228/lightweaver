use super::Location;



pub struct BoundingBox {
    pub top_left:     Location,
    pub bottom_right: Location,
}

pub trait GetBounding {
    fn bounding_box(&self) -> BoundingBox;
}

impl BoundingBox {

    pub fn left(&self) -> f64 {
        self.top_left.x
    }

    pub fn right(&self) -> f64 {
        self.bottom_right.x
    }

    pub fn top(&self) -> f64 {
        self.top_left.y
    }

    pub fn bottom(&self) -> f64 {
        self.bottom_right.y
    }

    pub fn top_right(&self) -> Location {
        Location::new(
            self.right(),
            self.top(),
        )
    }

    pub fn bottom_left(&self) -> Location {
        Location::new(
            self.left(),
            self.bottom(),
        )
    }
}

type Points<'a> = (&'a Vec<f64>, &'a Vec<f64>);

impl From<Points<'_>> for BoundingBox {
    fn from(points: Points) -> BoundingBox {

        let x_list = points.0;
        let y_list = points.1;

        let min_x = x_list.iter().reduce(|acc, x| if acc < x {acc} else {x}).unwrap();
        let min_y = y_list.iter().reduce(|acc, y| if acc < y {acc} else {y}).unwrap();
        let max_x = x_list.iter().reduce(|acc, x| if acc > x {acc} else {x}).unwrap();
        let max_y = y_list.iter().reduce(|acc, y| if acc > y {acc} else {y}).unwrap();

        BoundingBox {
            top_left:     Location::new(*min_x, *max_y),
            bottom_right: Location::new(*max_x, *min_y),
        }
    }
}

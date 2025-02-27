pub struct Dimensions {
    pub width:  f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,

    #[allow(dead_code)]
    pub z: f64,
}

pub struct BoundingBox {
    pub top_left:     Location,
    pub bottom_right: Location,
}


pub struct Rect {
    pub center: Location,
    pub dim:    Dimensions,
}

pub struct Line {
    pub start: Location,
    pub end:   Location,
    
}

type Vertices = (Location, Location, Location);

pub struct Triangle {
    pub vertices: Vertices,
}


impl BoundingBox {

    pub fn from_vecs(x_list: &Vec<f64>, y_list: &Vec<f64>) -> BoundingBox {
        let min_x = x_list.iter().reduce(|acc, x| if acc < x {acc} else {x}).unwrap();
        let min_y = y_list.iter().reduce(|acc, y| if acc < y {acc} else {y}).unwrap();
        let max_x = x_list.iter().reduce(|acc, x| if acc > x {acc} else {x}).unwrap();
        let max_y = y_list.iter().reduce(|acc, y| if acc > y {acc} else {y}).unwrap();

        BoundingBox {
            top_left:     Location::new(min_x.clone(), max_y.clone()),
            bottom_right: Location::new(max_x.clone(), min_y.clone()),
        }
    }

    pub fn left(&self) -> f64 {
        self.top_left.x.clone()
    }

    pub fn right(&self) -> f64 {
        self.bottom_right.x.clone()
    }

    pub fn top(&self) -> f64 {
        self.top_left.y.clone()
    }

    pub fn bottom(&self) -> f64 {
        self.bottom_right.y.clone()
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

// TODO: add in bounds checking for [-1, 1]

impl Dimensions {
    pub fn new(width: f64, height: f64) -> Dimensions {
        Dimensions {
            width,
            height,
        }
    }
}

impl Location {
    pub fn new(x: f64, y: f64) -> Location {
        Location { 
            x,
            y,
            z: 0.0,
        }
    }
}

impl Rect {
    pub fn new(center: Location, dim: Dimensions) -> Rect {
        Rect {
            center,
            dim,
        }
    }

    pub fn from_box(bounding_box: &BoundingBox) -> Rect {

        let dim = Dimensions::new(
            bounding_box.right() - bounding_box.left(),
            bounding_box.top()   - bounding_box.bottom(),
        );

        let center = Location::new(
            (dim.width  / 2.0) + bounding_box.left(),
            (dim.height / 2.0) + bounding_box.bottom(),

        );

        Rect {
            center,
            dim,
        }

    }

    pub fn bounding_box(&self) -> BoundingBox {

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

impl Line {
    pub fn new(start: Location, end: Location) -> Line {
        Line {
            start,
            end,
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let x = vec![self.start.x, self.end.x];
        let y = vec![self.start.y, self.end.y];

        BoundingBox::from_vecs(&x, &y)
    }
}

impl Triangle {
    pub fn new(vertices: Vertices) -> Triangle {
        Triangle {
            vertices,
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {

        let x = vec![self.vertices.0.x, self.vertices.1.x, self.vertices.2.x];
        let y = vec![self.vertices.0.y, self.vertices.1.y, self.vertices.2.y];

        BoundingBox::from_vecs(&x, &y)
    }
}
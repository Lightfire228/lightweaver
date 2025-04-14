mod rect;
mod line;
mod triangle;
mod bounding_box;

pub use rect::Rect;
pub use line::Line;
pub use triangle::Triangle;
pub use bounding_box::{{BoundingBox, GetBounding}};

pub struct Dimensions {
    pub width:  f64,
    pub height: f64,
}

impl Dimensions {
    pub fn new(width: f64, height: f64) -> Dimensions {
        Dimensions {
            width,
            height,
        }
    }
}

#[derive(Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,

    #[allow(dead_code)]
    pub z: f64,
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

// TODO:
#[allow(unused)]
pub enum ShapeType {
    Rect    (Rect),
    Line    (Line),
    Triangle(Triangle),
}
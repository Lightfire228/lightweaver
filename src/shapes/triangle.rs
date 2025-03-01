#![allow(dead_code)]

use super::{BoundingBox, Location};


type Vertices = (Location, Location, Location);

pub struct Triangle {
    pub vertices: Vertices,
}

pub trait GetBounding {
    fn bounding_box(&self) -> BoundingBox;
}


impl Triangle {
    pub fn new(vertices: Vertices) -> Triangle {
        Triangle {
            vertices,
        }
    }
}

impl GetBounding for Triangle {
    fn bounding_box(&self) -> BoundingBox {

        let x = vec![self.vertices.0.x, self.vertices.1.x, self.vertices.2.x];
        let y = vec![self.vertices.0.y, self.vertices.1.y, self.vertices.2.y];

        BoundingBox::from((&x, &y))
    }
}
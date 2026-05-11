use cgmath::{vec2, vec3};

use crate::app::{Index, Vertex};

#[derive(Debug)]
pub enum Shape {
    Rect(Rect),
}


#[derive(Debug)]
pub struct Rect {}

pub struct Quad {
    pub vertices: [Vertex; 4],
    pub indices:  [Index; 6]
}

impl Shape {
    pub fn as_quad(&self, i: usize) -> Quad {
        match self {
            Shape::Rect(rect) => rect.as_quad(i),
        }
    }
}

impl Rect {

    pub fn as_quad(&self, i: usize) -> Quad {
        let i = i as u32;
        Quad {
            vertices: [
                Vertex::new(vec3(-0.5, -0.5, -0.3*i as f32), vec3(1.0, 0.0, 0.0), vec2(1.0, 0.0)),
                Vertex::new(vec3( 0.5, -0.5, -0.3*i as f32), vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
                Vertex::new(vec3( 0.5,  0.5, -0.3*i as f32), vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
                Vertex::new(vec3(-0.5,  0.5, -0.3*i as f32), vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
            ],
            indices: [
                0 + i * 4,
                1 + i * 4,
                2 + i * 4,
                2 + i * 4,
                3 + i * 4,
                0 + i * 4,
            ],
        }
    }
}

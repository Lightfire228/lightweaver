use cgmath::{vec2, vec3};

use crate::app::{Index, Vertex};

#[derive(Debug)]
pub enum Shape {
    Rect(Rect),
    Cube(Cube),
}


#[derive(Debug)]
pub struct Rect {}

#[derive(Debug)]
pub struct Cube {}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices:  Vec<Index>,
}

impl Shape {
    pub fn as_quads(&self) -> Mesh {
        match self {
            Shape::Rect(rect) => rect.into(),
            Shape::Cube(cube) => cube.into(),
        }
    }
}


impl From<&Rect> for Mesh {
    fn from(_: &Rect) -> Self {
        Mesh {
            vertices: vec![
                Vertex::new(vec3(-1.0, -1.0, 0.0), vec3(1.0, 0.0, 0.0), vec2(1.0, 0.0)),
                Vertex::new(vec3( 1.0, -1.0, 0.0), vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
                Vertex::new(vec3( 1.0,  1.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
                Vertex::new(vec3(-1.0,  1.0, 0.0), vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
            ],
            indices: vec![
                0,
                1,
                2,
                2,
                3,
                0,
            ],
        }
    }
}

impl From<Rect> for Mesh {
    fn from(value: Rect) -> Self {
        (&value).into()
    }
}


impl From<&Cube> for Mesh {
    fn from(_: &Cube) -> Self {

        let left   =  1.0;
        let right  = -1.0;
        let bottom = -1.0;
        let top    =  1.0;
        let back   = -1.0;
        let front  =  1.0;

        let mut i = 0;
        let mut inc = || {i += 1; i-1};

        let left_bottom_back   = inc();
        let left_bottom_front  = inc();
        let left_top_back      = inc();
        let left_top_front     = inc();
        let right_bottom_back  = inc();
        let right_bottom_front = inc();
        let right_top_back     = inc();
        let right_top_front    = inc();

        // rubiks cube colors
        let red    = vec3(0.0,  0.0,  0.0);
        let white  = vec3(1.0,  1.0,  1.0);
        let blue   = vec3(0.0,  0.0,  1.0);
        let orange = vec3(1.0, 0.25, 0.06);
        let green  = vec3(0.0,  1.0,  0.0);
        let yellow = vec3(1.0,  1.0,  0.0);

        Mesh {
            vertices: vec![
                // left face
                Vertex::new(vec3(left,  bottom, back),  red, vec2(0.0, 1.0)),
                Vertex::new(vec3(left,  bottom, front), red, vec2(0.0, 1.0)),
                Vertex::new(vec3(left,  top,    back),  red, vec2(0.0, 0.0)),
                Vertex::new(vec3(left,  top,    front), red, vec2(0.0, 0.0)),

                // right face
                Vertex::new(vec3(right, bottom, back),  orange, vec2(0.0, 1.0)),
                Vertex::new(vec3(right, bottom, front), orange, vec2(0.0, 1.0)),
                Vertex::new(vec3(right, top,    back),  orange, vec2(0.0, 0.0)),
                Vertex::new(vec3(right, top,    front), orange, vec2(0.0, 0.0)),

                // top face
                Vertex::new(vec3(left,  top, back),     white, vec2(1.0, 1.0)),
                Vertex::new(vec3(left,  top, front),    white, vec2(1.0, 1.0)),
                Vertex::new(vec3(right, top, front),    white, vec2(1.0, 0.0)),
                Vertex::new(vec3(right, top, back),     white, vec2(1.0, 0.0)),

                // bottom face
                Vertex::new(vec3(left,  bottom, back),  yellow, vec2(1.0, 1.0)),
                Vertex::new(vec3(left,  bottom, front), yellow, vec2(1.0, 1.0)),
                Vertex::new(vec3(right, bottom, front), yellow, vec2(1.0, 0.0)),
                Vertex::new(vec3(right, bottom, back),  yellow, vec2(1.0, 0.0)),

                // front face
                Vertex::new(vec3(right, top,    front), blue, vec2(1.0, 0.0)),
                Vertex::new(vec3(left,  top,    front), blue, vec2(1.0, 1.0)),
                Vertex::new(vec3(right, bottom, front), blue, vec2(1.0, 0.0)),
                Vertex::new(vec3(left,  bottom, front), blue, vec2(1.0, 1.0)),

                // back face
                Vertex::new(vec3(left,  top,    back),  green, vec2(1.0, 1.0)),
                Vertex::new(vec3(left,  bottom, back),  green, vec2(1.0, 1.0)),
                Vertex::new(vec3(right, bottom, back),  green, vec2(1.0, 0.0)),
                Vertex::new(vec3(right, top,    back),  green, vec2(1.0, 0.0)),
            ],
            // pipeline culls backside, front face set in pipeline
            // vk::FrontFace::COUNTER_CLOCKWISE
            indices: vec![
                // left face
                0, 1, 2, 2, 0, 3,

                // right face
                4, 5, 6, 6, 4, 7,

                // top face
                9, 8, 10, 10, 8, 11,

                // bottom face
                12, 13, 14, 14, 15, 12,

                // front face
                16, 19, 17, 18, 19, 16,

                // back face
                20, 21, 22, 22, 23, 20,



                // // back face
                // left_bottom_back, left_top_back, right_bottom_back

            ],
        }
    }
}

impl From<Cube> for Mesh {
    fn from(value: Cube) -> Self {
        (&value).into()
    }
}

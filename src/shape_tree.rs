#![allow(unused)]

use crate::shapes::{ShapeType};

pub struct ShapeTree {
    root: ShapeTreeNode,
}

impl ShapeTree {

    pub fn new() -> Self {

        Self {
            root: ShapeTreeNode::new(),
        }
    }

    pub fn add_shape(&mut self, shape: ShapeType) {
        self.root.add_shape(shape);
    }

}

pub enum NodeOrShape {
    Node (ShapeTreeNode),
    Shape(ShapeType),
}

pub struct ShapeTreeNode {
    pub children:  Vec<NodeOrShape>,
}

impl ShapeTreeNode {
    pub fn new() -> ShapeTreeNode {

        ShapeTreeNode {
            children: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: ShapeType) {
        let shape = NodeOrShape::Shape(shape);
        self.children.push(shape);
    }

}


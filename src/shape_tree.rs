use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};

use crate::shapes::ShapeType;

pub enum Direction {
    HORT,
    VERT,
}

pub enum NodeOrShape<'a> {
    Node (RefCell<ShapeTreeNode<'a>>),
    Shape(ShapeType),
}

pub struct ShapeTreeNode<'a> {
    pub parent:    Option<Ref<'a, ShapeTreeNode<'a>>>,
    pub direction: Direction,

    pub children:  Vec<NodeOrShape<'a>>,
}

impl ShapeTreeNode<'_> {
    pub fn new<'a>(direction: Direction) -> RefCell<ShapeTreeNode<'a>> {

        RefCell::new(
            ShapeTreeNode {
                parent: None,
                direction,

                children: vec![],
            }
        )
    }

    fn add_group<'a>(parent: &RefCell<ShapeTreeNode>, direction: Direction) -> Ref<'a, ShapeTreeNode<'a>> {

        use self::Direction::*;

        let mut p = parent.borrow_mut();

        let dir = match p.direction {
            HORT => VERT,
            VERT => HORT,
        };

        let node = RefCell::new(
            ShapeTreeNode {
                parent: Some(parent.borrow()),
                direction,

                children: vec![],
            }
        );

        p.children.push(NodeOrShape::Node(node));

        node.borrow()
    }

    pub fn add(&mut self, shape: ShapeType) {
        let shape = NodeOrShape::Shape(shape);
        self.children.push(shape);

    }

}


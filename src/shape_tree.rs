use std::rc::{Rc, Weak};

use crate::shapes::ShapeType;

pub enum Direction {
    HORT,
    VERT,
}

pub enum NodeOrShape {
    Node (Rc<ShapeTreeNode>),
    Shape(ShapeType),
}

pub struct ShapeTreeNode {
    pub parent:    Option<Rc<ShapeTreeNode>>,
    pub direction: Direction,

    pub children:  Vec<NodeOrShape>,

    me: Weak<ShapeTreeNode>
}

impl ShapeTreeNode {
    pub fn new(direction: Direction) -> Rc<ShapeTreeNode> {

        Rc::new_cyclic(|new_| 
            ShapeTreeNode {
                parent: None,
                direction,

                children: vec![],
                me: new_.clone(),
            }
        )
    }

    fn with_parent(parent: &Rc<ShapeTreeNode>, direction: Direction) -> Rc<ShapeTreeNode> {

        Rc::new_cyclic(|new_| 
            ShapeTreeNode {
                parent: Some(parent.clone()),
                direction,

                children: vec![],
                me: new_.clone(),
            }
        )
    }

    pub fn me(&self) -> Rc<Self> {
        self.me.upgrade().unwrap()
    }

    pub fn add(&mut self, shape: ShapeType) {
        let shape = NodeOrShape::Shape(shape);
        self.children.push(shape);

    }

    pub fn add_group(&mut self) {

        use self::Direction::*;

        let dir = match self.direction {
            HORT => VERT,
            VERT => HORT,
        };

        let node = NodeOrShape::Node(Self::with_parent(&self.me(), dir));

        self.children.push(node);
    }
}


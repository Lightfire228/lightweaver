use crate::shapes;

pub struct ShapeTreeCoordinates {
    x: f64,
    y: f64,
}


pub trait ShapeElement {

    fn location(&self) -> &mut ShapeTreeCoordinates;
}

pub type ShapeList = Vec<NodeElement>;


pub struct NodeElement {
    pub location: ShapeTreeCoordinates,
    pub element:  Box<dyn ShapeElement>,
}

pub enum Direction {
    HORT,
    VERT,
}


pub struct ShapeTreeNode<'a> {
    pub parent:    Option<&'a ShapeTreeNode<'a>>,
    pub direction: Direction,

    pub shapes:    ShapeList,
    pub children:  Vec<ShapeTreeNode<'a>>,
}

impl ShapeTreeNode<'_> {
    pub fn new<'a>(direction: Direction) -> ShapeTreeNode<'a> {

        ShapeTreeNode {
            parent: None,
            direction,

            shapes:   vec![],
            children: vec![],
        }
    }

    pub fn with_parent<'a>(parent: &'a ShapeTreeNode<'_>, direction: Direction) -> ShapeTreeNode<'a> {
        ShapeTreeNode {
            parent: Some(parent),
            direction,
    
            shapes:   vec![],
            children: vec![],
        }
    }

    pub fn calculate_verticies(&mut self, scale: f64) {
        let size = self.size();

        let x = 0.0;
        let y = 0.0;

        // for this to work, "children" and "shapes" need to be the same iter
        // (or their ordinals need set)
        for el in self.children.iter() {
        }
        
    }

    pub fn size(&self) -> usize {
        self.shapes.len() + self.children.iter().map(|x| x.size()).sum::<usize>()
    


}

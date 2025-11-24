

// https://doc.rust-lang.org/src/alloc/collections/btree/map.rs.html#173-177
// https://blog.logrocket.com/guide-using-arenas-rust/
// https://rust-unofficial.github.io/too-many-lists/

// TODO:
#![allow(unused)]

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GraphTree<T> {
    children: Vec<Link<T>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Child<T> {
    LeafNode(T),
    SubTree(GraphTree<T>)
}

type Link<T> = Box<Child<T>>;


impl<T> GraphTree<T> {
    pub fn new() -> Self {
        GraphTree {
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.children.push(
            Box::new(Child::LeafNode(elem))
        );
    }

    pub fn push_tree(&mut self, node: GraphTree<T>) {
        self.children.push(
            Box::new(Child::SubTree(node))
        );
    }

    pub fn peek(&self) -> Option<&Child<T>> {
        self.children.last().map(|c| {
            c.as_ref()
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut Child<T>> {
        self.children.last_mut().map(|c| {
            c.as_mut()
        })
    }

    /// this recursively looks through the last child of each node
    /// and returns the last element contained in it
    pub fn peek_deep(&self) -> Option<&T> {

        self.children.last().map(|c| {
            match c.as_ref() {
                Child::LeafNode(l)  => Some(l),
                Child::SubTree(sub) => sub.peek_deep()
            }
        })?
    }

    /// this recursively looks through the last child of each node
    /// and returns the last element contained in it
    pub fn peek_deep_mut(&mut self) -> Option<&mut T> {

        self.children.last_mut().map(|c| {
            match c.as_mut() {
                Child::LeafNode(l)  => Some(l),
                Child::SubTree(sub) => sub.peek_deep_mut()
            }
        })?
    }

}

// === Iters

pub struct IntoIter<T>(Child<T>);

pub struct Iter<'a, T> {

    next: Option<&'a GraphTree<T>>
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut GraphTree<T>>
}


impl<T> GraphTree<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(Child::SubTree(self))
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: Some(&self) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: Some(self) }
    }
}


// === Tests

#[cfg(test)]
mod test {
    use super::{GraphTree, Child::*};

    #[test]
    fn base() {
        let mut root = GraphTree::new();

        root.push(1);
        root.push(2);
        root.push(3);

        assert_eq!(root.peek(),          Some(&    LeafNode(3)));
        assert_eq!(root.peek_mut(),      Some(&mut LeafNode(3)));
        assert_eq!(root.peek_deep(),     Some(&    3));
        assert_eq!(root.peek_deep_mut(), Some(&mut 3));

        // test peek mutability
        let a = root.peek_mut();
        match a.unwrap() {
            LeafNode(node) => { *node = 4 },
            SubTree(_)     => assert!(false)
        }
        assert_eq!(root.peek_mut(), Some(&mut LeafNode(4)));

        // test deep peek mutability
        let a = root.peek_deep_mut();
        *a.unwrap() = 5;

        assert_eq!(root.peek_deep_mut(), Some(&mut 5));


        dbg!("{}", root);
    }

    #[test]
    fn add_subtree() {
        let mut root = GraphTree::new();

        root.push(1);
        root.push(2);

        let mut node = GraphTree::new();
        node.push(4);
        node.push(5);

        let n = node.clone();
        root.push_tree(node);

        assert_eq!(root.peek(),      Some(&SubTree(n)));
        assert_eq!(root.peek_deep(), Some(&5));

        root.push(3);

        assert_eq!(root.peek(),      Some(&LeafNode(3)));
        assert_eq!(root.peek_deep(), Some(&3));
    }

    #[test]
    fn print_node() {
        let mut root = GraphTree::new();

        root.push(1);
        root.push(2);
        root.push(3);

        dbg!("{}", root);
        // assert!(false); // make it print
    }
}

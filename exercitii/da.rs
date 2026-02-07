use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn insert(&mut self, new_value: T) {
        match new_value.cmp(&self.value) {
            Ordering::Less => {
                match self.left {
                    Some(ref mut left_node) => left_node.insert(new_value),
                    None => {
                        self.left = Some(Box::new(Node {
                            value: new_value,
                            left: None,
                            right: None,
                        }))
                    }
                }
            }
            Ordering::Greater => {
                match self.right {
                    Some(ref mut right_node) => right_node.insert(new_value),
                    None => {
                        self.right = Some(Box::new(Node {
                            value: new_value,
                            left: None,
                            right: None,
                        }))
                    }
                }
            }
            Ordering::Equal => {}
        }
    }

    fn find(&self, target: &T) -> bool {
        match target.cmp(&self.value) {
            Ordering::Less => {
                match &self.left {
                    Some(left) => left.find(target),
                None => false,
                }
            }
            Ordering::Greater => {
                match &self.right {
                    Some(right) => right.find(target),
                    None => false,
                }
            }
            Ordering::Equal => true,
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {root: None}
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => {
                self.root = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    pub fn find(&self, value: T) -> bool {
        match &self.root {
            Some(root) => root.find(&value),
            None => false,
        }
    }
}
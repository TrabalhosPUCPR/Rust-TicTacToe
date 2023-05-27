use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>,
    pub utility: i32,
    pub heuristic: f32
}

impl<T> Node<T> {
    pub fn get_true_utility(&self) -> f32 {
        self.utility as f32 + (self.heuristic as f32 / 1000.0)
    }
    pub fn is_terminal(&self) -> bool {
        self.children.is_empty()
    }
}

impl<T> Node<T> where T: Clone + PartialEq {
    pub fn new(data: T) -> Node<T> {
        Node{
            data,
            children: vec![],
            utility: Default::default(),
            heuristic: Default::default(),
        }
    }
    pub fn get_children_sorted(&self, max: bool) -> Vec<Node<T>> {
        let mut children = self.children.clone();
        if !max {
            children.sort_unstable_by(|a, b| { a.partial_cmp(b).unwrap() });
        }else {
            children.sort_unstable_by(|a, b| { b.partial_cmp(a).unwrap() });
        }
        children
    }
}

impl<T> PartialOrd for Node<T> where T: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_true_utility().total_cmp(&other.get_true_utility()))
    }

    fn lt(&self, other: &Self) -> bool {
        Node::get_true_utility(self) < Node::get_true_utility(other)
    }

    fn le(&self, other: &Self) -> bool {
        Node::get_true_utility(self) <= Node::get_true_utility(other)
    }

    fn gt(&self, other: &Self) -> bool {
        Node::get_true_utility(self) > Node::get_true_utility(other)
    }

    fn ge(&self, other: &Self) -> bool {
        Node::get_true_utility(self) >= Node::get_true_utility(other)
    }
}
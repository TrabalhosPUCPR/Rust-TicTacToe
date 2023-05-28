use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>,
    pub data_score: f32
}

impl<T> Node<T> {
    pub fn is_terminal(&self) -> bool {
        self.children.is_empty()
    }
}

impl<T> Node<T> where T: Clone + PartialEq {
    pub fn new(data: T) -> Node<T> {
        Node{
            data,
            children: vec![],
            data_score: Default::default()
        }
    }
}

impl<T> PartialOrd for Node<T> where T: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data_score.partial_cmp(&other.data_score)
    }

    fn lt(&self, other: &Self) -> bool {
        self.data_score < other.data_score
    }

    fn le(&self, other: &Self) -> bool {
        self.data_score <= other.data_score
    }

    fn gt(&self, other: &Self) -> bool {
        self.data_score > other.data_score
    }

    fn ge(&self, other: &Self) -> bool {
        self.data_score >= other.data_score
    }
}
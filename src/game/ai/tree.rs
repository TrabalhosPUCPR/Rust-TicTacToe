use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>,
    pub utility: i32,
    pub heuristic: i32
}

impl<T> PartialOrd for Node<T> where T: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.utility.cmp(&other.utility))
    }

    fn lt(&self, other: &Self) -> bool {
        self.utility < other.utility
    }

    fn le(&self, other: &Self) -> bool {
        self.utility <= other.utility
    }

    fn gt(&self, other: &Self) -> bool {
        self.utility > other.utility
    }

    fn ge(&self, other: &Self) -> bool {
        self.utility >= other.utility
    }
}

#[derive(Clone)]
enum MinMaxNode<T> {
    Alpha(Option<Node<T>>),
    Beta(Option<Node<T>>),
}

impl<T> MinMaxNode<T> where T: Clone + PartialEq{
    fn unwrap(&self) -> Option<Node<T>>{
        return match self {
            MinMaxNode::Alpha(n) => {
                n.to_owned()
            }
            MinMaxNode::Beta(n) => {
                n.to_owned()
            }
        }
    }
    fn full_unwrap(&self) -> Node<T>{
        return match self {
            MinMaxNode::Alpha(n) => {
                n.to_owned().unwrap()
            }
            MinMaxNode::Beta(n) => {
                n.to_owned().unwrap()
            }
        }
    }
    fn from(max: bool) -> MinMaxNode<T> {
        return if max {
            MinMaxNode::Alpha(None)
        }else {
            MinMaxNode::Beta(None)
        }
    }
    fn is_none(&self) -> bool {
        return match self {
            MinMaxNode::Alpha(n) => n.is_none(),
            MinMaxNode::Beta(n) => n.is_none()
        }
    }
    fn is_some(&self) -> bool {
        return match self {
            MinMaxNode::Alpha(n) => n.is_some(),
            MinMaxNode::Beta(n) => n.is_some()
        }
    }
    fn verify(&self, found_node: Node<T>) -> bool {
        match self.clone() {
            MinMaxNode::Alpha(value) => {
                if value.is_none() || value.clone().unwrap() < found_node {
                    return true
                }
            }
            MinMaxNode::Beta(value) => {
                if value.is_none() || value.clone().unwrap() > found_node {
                    return true
                }
            }
        }
        false
    }
    fn verify_and_set(&mut self, found_node: Node<T>) -> bool {
        match self.clone() {
            MinMaxNode::Alpha(value) => {
                if value.is_none() || value.clone().unwrap() < found_node {
                    *self = MinMaxNode::Alpha(Some(found_node));
                    return true
                }
            }
            MinMaxNode::Beta(value) => {
                if value.is_none() || value.clone().unwrap() > found_node {
                    *self = MinMaxNode::Beta(Some(found_node));
                    return true
                }
            }
        }
        false
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
    pub fn is_terminal(&self) -> bool {
        self.children.is_empty()
    }
    pub fn minimax(&mut self, max: bool) -> Node<T> {
        let mut node_type;
        if max {
            node_type = MinMaxNode::Alpha(None);
        }else {
            node_type = MinMaxNode::Beta(None);
        }
        let result = self.get_max_min_child_utility(&mut node_type, max);
        if let Some(next) = result.1 {
            next
        }else {
            panic!("aaa")
        }
    }
    fn get_max_min_child_utility(&mut self, parent_node_type: &mut MinMaxNode<T>, max: bool) -> (Option<Node<T>>, Option<Node<T>>) {
        if self.is_terminal() {
            return (Some(self.clone()), None)
        }
        let mut self_node_type = MinMaxNode::from(max);
        for child in self.children.iter_mut() {
            let next;
            if parent_node_type.is_none() {
                next = child.get_max_min_child_utility(&mut self_node_type, !max);
            }else {
                next = child.get_max_min_child_utility(parent_node_type, !max);
            }
            if let Some(n) = next.0 {
                self_node_type.verify_and_set(n);
            }
        }
        if self_node_type.is_some() && parent_node_type.verify(self_node_type.clone().full_unwrap()) {
            self.utility = self_node_type.full_unwrap().utility;
            return (Some(self.to_owned()), self_node_type.unwrap())
        }
        (None, None)
    }
}

#[derive(Clone)]
pub struct Tree<T> {
    pub root: Option<Node<T>>,
}

impl<T> Tree<T> where T: Clone + PartialEq {
    pub fn new() -> Tree<T> {
        Tree {
            root: None,
        }
    }
    pub fn add_root(&mut self, data: T) {
        self.root = Some(Node::new(data))
    }
    pub fn set_root(&mut self, node: Node<T>) {
        self.root = Some(node)
    }
}

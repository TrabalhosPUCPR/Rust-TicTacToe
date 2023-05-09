#[derive(Clone)]
pub struct Node<T> {
    pub childs: Vec<Node<T>>,
    pub data: T
}

pub trait Utility {
    fn get_utility(&self) -> i32;
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T>{
        Node {
            childs: vec![],
            data,
        }
    }
    pub fn add_child(&mut self, data: T) {
        self.childs.push(Node::new(data))
    }
    pub fn is_terminal(&self) -> bool {
        self.childs.is_empty()
    }

    pub fn minimax(node: &Node<T>) -> &Node<T> where T:Utility {
        node.get_max_child_utility()
    }

    fn get_max_child_utility(&self) -> &Node<T> where T: Utility{
        if self.is_terminal() {
            return self
        }
        let mut iterator = self.childs.iter();
        let mut next = iterator.next();
        let mut final_node = next.unwrap().get_min_child_utility();
        next = iterator.next();
        while next.is_some() {
            let node = next.unwrap().get_min_child_utility();
            if final_node.data.get_utility() < node.data.get_utility() {
                final_node = self;
            }
            next = iterator.next();
        }
        final_node
    }

    fn get_min_child_utility(&self) -> &Node<T> where T: Utility{
        if self.is_terminal() {
            return self
        }
        let mut iterator = self.childs.iter();
        let mut next = iterator.next();
        let mut final_node = next.unwrap().get_max_child_utility();
        next = iterator.next();
        while next.is_some() {
            let node = next.unwrap().get_max_child_utility();
            if final_node.data.get_utility() > node.data.get_utility() {
                final_node = self;
            }
            next = iterator.next();
        }
        final_node
    }
}

#[derive(Clone)]
pub struct Tree<T> {
    pub root: Option<Node<T>>,
    pub size: u32,
    pub layers: u32,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root: None,
            size: 1,
            layers: 1,
        }
    }
    pub fn set_root(&mut self, data: T) {
        self.root = Some(Node::new(data))
    }
    pub fn add_as_root(&mut self, node: Node<T>) {
        self.root = Some(node)
    }
}

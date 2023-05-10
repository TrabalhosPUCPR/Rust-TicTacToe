#[derive(Clone)]
pub struct Node<T> {
    pub childs: Vec<Node<T>>,
    pub data: T,
    pub utility: Option<i32>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            childs: vec![],
            data,
            utility: None
        }
    }
    pub fn add_child(&mut self, data: T) {
        self.childs.push(Node::new(data))
    }
    pub fn is_terminal(&self) -> bool {
        self.childs.is_empty()
    }

    pub fn minimax(&self, max: bool) -> Node<T> where T: Clone {
        let self_binding = self.clone();
        let mut best_move: Option<Node<T>> = None;
        for mut c in self_binding.childs {
            if max {
                let m = c.get_min_child_utility();
                if best_move.is_none() || m.utility > best_move.clone().unwrap().utility {
                    c.utility = m.utility;
                    best_move = Some(c);
                }
            }else {
                let m = c.get_max_child_utility();
                if best_move.is_none() || m.utility < best_move.clone().unwrap().utility {
                    c.utility = m.utility;
                    best_move = Some(c);
                }
            }
        }
        best_move.unwrap()
    }

    fn get_max_child_utility(&self) -> Node<T> where T: Clone {
        if self.is_terminal() {
            return self.clone()
        }
        let mut iterator = self.childs.iter();
        let mut next = iterator.next();
        let mut final_node = next.unwrap().get_min_child_utility();
        next = iterator.next();
        while next.is_some() {
            let node = next.unwrap().get_min_child_utility();
            if final_node.utility < node.utility {
                final_node = self.clone();
                final_node.utility = node.utility
            }
            next = iterator.next();
        }
        final_node
    }

    fn get_min_child_utility(&self) -> Node<T> where T: Clone {
        if self.is_terminal() {
            return self.clone()
        }
        let mut iterator = self.childs.iter();
        let mut next = iterator.next();
        let mut final_node = next.unwrap().get_max_child_utility();
        next = iterator.next();
        while next.is_some() {
            let node = next.unwrap().get_max_child_utility();
            if final_node.utility > node.utility {
                final_node = self.clone();
                final_node.utility = node.utility
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

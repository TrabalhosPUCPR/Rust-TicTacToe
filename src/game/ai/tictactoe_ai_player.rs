use crate::game::ai::node::Node;
use crate::game::tictactoe_core::{SpecialBoardChecks, SquareState, TicTacToe, TurnState};

#[derive(Clone)]
pub struct Ai{
    max_node_childs: usize,
    max_layers: usize,
    pub symbol: char,
    pub op_symbol: char
}

#[derive(Clone, PartialEq)]
enum MinMaxNode<T> {
    Alpha(Option<Node<T>>),
    Beta(Option<Node<T>>),
}

impl<T> MinMaxNode<T> where T: Clone + PartialEq{
    fn unwrap(self) -> Option<Node<T>>{
        return match self {
            MinMaxNode::Alpha(n) => {
                n
            }
            MinMaxNode::Beta(n) => {
                n
            }
        }
    }
    fn full_unwrap(self) -> Node<T>{
        return match self {
            MinMaxNode::Alpha(n) => {
                n.unwrap()
            }
            MinMaxNode::Beta(n) => {
                n.unwrap()
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

impl Ai {
    pub fn create(max_node_childs: usize, max_layers: usize, symbol: char, op_symbol: char) -> Ai {
        Ai {
            max_node_childs,
            max_layers,
            symbol,
            op_symbol
        }
    }
    pub fn act(&mut self, current_board: TicTacToe) -> (usize, usize) {
        let mut root = Node::new((current_board.clone(), 0, TurnState::Continue));
        self.load_moves_tree(&mut root, 1, true);

        let next_action = root.minimax(true);
        let coord = current_board.get_index_coord(next_action.data.1);
        if self.max_node_childs > 0 {
            self.max_node_childs += 2;
        }
        return coord
    }
    fn load_moves_tree(&self, current: &mut Node<(TicTacToe, usize, TurnState)>, layer: usize, own_turn: bool) {
        for square in current.data.0.squares.iter().enumerate() {
            if let SquareState::None = square.1 {
                let mut possible_move = current.data.clone();
                possible_move.1 = square.0;
                let move_state: TurnState;
                let square_state;
                if own_turn {
                    square_state = SquareState::Filled(self.symbol);
                }else {
                    square_state = SquareState::Filled(self.op_symbol);
                }
                move_state = possible_move.0.set_square_from_index(square.0, square_state);
                let mut possible_move_node = Node::new(possible_move);
                match move_state {
                    TurnState::Draw => {
                        possible_move_node.utility = 0;
                    }
                    TurnState::Victory => {
                        if own_turn {
                            possible_move_node.utility = 1;
                        }else {
                            possible_move_node.utility = -1;
                        }
                    }
                    TurnState::Continue => {
                        possible_move_node.heuristic = self.get_move_heuristic(&possible_move_node.data.0, square_state, square.0, own_turn)
                    }
                    _ => {
                        panic!("Unexpected Ai Error")
                    }
                }
                possible_move_node.data.2 = move_state;
                current.children.push(possible_move_node);
            }
        }
        let mut children = current.get_children_sorted(own_turn);
        if self.max_node_childs > 0 && children.len() > self.max_node_childs {
            children = children.split_at(self.max_node_childs).0.to_owned();
        }
        for c in children.iter_mut() {
            if (self.max_layers == 0 || layer + 1 <= self.max_layers) && c.data.2 == TurnState::Continue {
                self.load_moves_tree(c, layer + 1, !own_turn);
            }
        }
        current.children = children;
        current.children = vec![current.minimax(own_turn)]
    }

    pub fn get_move_heuristic(&self, board: &TicTacToe, square_state: SquareState, index: usize, own_turn: bool) -> f32 {
        /*
        THE FULL HEURISTIC IS BASED ON:
            - Amount of same symbols in winnable distance in same line, column and diagonals
            - Amount of symbols from oponent blocked
            - Amount of available axis to play after the move, that way he prioritizes spaces in the middle->cornes->center edges
            - Amount of empty spaces around the placed space
            - Position of the move, if its on the center edges, the empty spaces sum gets cleared, removing from the full score
        */
        let mut attack_score = board.sum_squares_in_winnable_distance(index, square_state, false) as f32;
        let mut available_axis = board.check_n_of_available_axis(index, square_state) as f32;
        let mut empty_space_around_score = board.spaces_of_around(index, SquareState::None) as i32;
        let op_symbol = if own_turn { SquareState::Filled(self.op_symbol) } else { SquareState::Filled(self.symbol) };
        let mut defense_score = board.sum_squares_in_winnable_distance(index, op_symbol, false) as f32;
        if board.seq_to_win.pow(2) < board.size() {
            empty_space_around_score = 0;
            if (board.seq_to_win % 2 != 0 && defense_score >= (board.seq_to_win as f32 / 2.0).ceil()) ||
                (board.seq_to_win % 2 == 0 && defense_score >= (board.seq_to_win as f32 - 2.0)) {
                defense_score *= 100.0;
            }
        }
        let x_size = board.x_size;
        let y_size = board.y_size;
        let coord = board.get_index_coord(index);
        if coord.1 == 0 || coord.1 == y_size - 1 {
            if coord.0 > 0 && coord.0 < x_size - 1 {
                empty_space_around_score = 0
            } else if coord.0 == 0 || coord.0 == x_size - 1 {
                empty_space_around_score = 0
            }
        }
        if !own_turn {
            attack_score *= -1.0;
            available_axis *= -1.0;
            empty_space_around_score *= -1;
            defense_score *= -1.0;
        }
        (attack_score + available_axis + (defense_score / 10.0)) + (empty_space_around_score as f32 / 100.0)
    }
}

impl<T> Node<T> where T: Clone + PartialEq {

    pub fn minimax(&mut self, max: bool) -> Node<T> {
        let mut node_type= MinMaxNode::from(max);
        let result = self.get_max_min_child_utility(&mut node_type, max);
        if let Some(next) = result.1 {
            next
        }else {
            panic!("Unexpected error, maybe the node does not have any children?")
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
                if &mut self_node_type != parent_node_type {
                    if !parent_node_type.verify(self_node_type.clone().full_unwrap()) {
                        return (None, None)
                    }
                }
            }
        }
        if self_node_type.is_some() && parent_node_type.verify(self_node_type.clone().full_unwrap()) {
            let found_node = self_node_type.full_unwrap();
            self.utility = found_node.utility;
            self.heuristic = found_node.heuristic;
            return (Some(self.to_owned()), Some(found_node))
        }
        (None, None)
    }
}

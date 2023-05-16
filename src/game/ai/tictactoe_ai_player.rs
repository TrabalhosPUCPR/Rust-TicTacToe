use crate::game::ai::node::Node;
use crate::game::tictactoe_core::{SquareState, TicTacToe, TurnState};

#[derive(Clone)]
pub struct Ai{
    max_node_childs: usize,
    max_layers: usize,
    pub symbol: char,
    pub op_symbol: char,
    pub debug_mode: bool,
}

impl Ai {
    pub fn create(max_node_childs: usize, max_layers: usize, symbol: char, op_symbol: char) -> Ai {
        Ai {
            max_node_childs,
            max_layers,
            symbol,
            op_symbol,
            debug_mode: false,
        }
    }
    pub fn act(&mut self, current_board: TicTacToe) -> (usize, usize) {
        let mut root = Node::new((current_board.clone(), 0, TurnState::Continue));
        self.load_moves_tree(&mut root, 1, true);

        let next_action = root.minimax(true);
        let coord = current_board.get_index_coord(next_action.data.1);
        return coord
    }
    fn load_moves_tree(&self, parent: &mut Node<(TicTacToe, usize, TurnState)>, layer: usize, own_turn: bool) {
        for square in parent.data.0.squares.iter().enumerate() {
            if let SquareState::None = square.1 {
                let mut possible_move = parent.data.clone();
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
                        /*
                            THE FULL HEURISTIC IS BASED ON:
                                - Amount of same symbols in winnable distance in same line, column and diagonals
                                - Amount of symbols from oponent blocked
                                - Amount of available axis to play after the move, that way he prioritizes spaces in the middle->cornes->center edges
                                - Amount of empty spaces around the placedspace
                                - Position of the move, if its on the center edges, the empty spaces sum gets cleared, removing from the full score
                        */
                        let mut squares_sum = possible_move_node.data.0.sum_of_same_squares_in_winnable_distance(square.0, square_state);
                        let mut available_axis = possible_move_node.data.0.check_n_of_available_axis(square.0, square_state) as i32;
                        let mut empty_sum = possible_move_node.data.0.empty_spaces_around(square.0) as i32;
                        let mut blocked = possible_move_node.data.0.check_blocked_op_spaces(square.0, SquareState::Filled(self.op_symbol)) as i32;

                        let x_size = possible_move_node.data.0.x_size;
                        let y_size = possible_move_node.data.0.y_size;
                        let coord = possible_move_node.data.0.get_index_coord(square.0);
                        if coord.1 == 0 || coord.1 == y_size - 1 {
                            if coord.0 > 0 && coord.0 < x_size-1 {
                                empty_sum = 0
                            }
                        }else if coord.0 == 0 || coord.0 == x_size-1 {
                            empty_sum = 0
                        }
                        if !own_turn {
                            squares_sum *= -1;
                            available_axis *= -1;
                            empty_sum *= -1;
                            blocked *= -1;
                        }
                        possible_move_node.heuristic = squares_sum as f32 + (blocked as f32 / 10.0) + ((empty_sum + available_axis) as f32 / 100.0);
                    }
                    _ => {
                        panic!("Unexpected Ai Error")
                    }
                }
                possible_move_node.data.2 = move_state;
                parent.children.push(possible_move_node);
            }
        }
        let mut children = parent.get_children_sorted(!own_turn);
        if self.max_node_childs > 0 && children.len() > self.max_node_childs {
            children = children.split_at(self.max_node_childs).0.to_owned();
        }
        for c in children.iter_mut() {
            if (self.max_layers == 0 || layer + 1 < self.max_layers) && c.data.2 == TurnState::Continue {
                self.load_moves_tree(c, layer + 1, !own_turn);
            }
        }
        parent.children = children
    }
}
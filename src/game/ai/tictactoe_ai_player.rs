use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::game::ai::node::Node;
use crate::game::tictactoe_core::{SpecialBoardChecks, SquareState, TicTacToe, TurnState};

#[derive(Clone)]
pub struct Ai{
    max_moves_to_analyze: usize,
    max_layers: usize,
    pub symbol: char,
    pub op_symbol: char
}

impl Ai {
    pub fn create(max_moves_to_analyze: usize, max_layers: usize, symbol: char, op_symbol: char) -> Ai {
        Ai {
            max_moves_to_analyze,
            max_layers,
            symbol,
            op_symbol
        }
    }
    pub fn act(&mut self, current_board: TicTacToe) -> (usize, usize) {
        let mut root = Node::new((current_board.clone(), 0, TurnState::Continue));
        let (_, index) = self.compute_next_move(&mut root, f32::NEG_INFINITY, f32::INFINITY, self.max_layers, true);
        return current_board.get_index_coord(index);
    }
    fn compute_next_move(&self, current_node: &Node<(TicTacToe, usize, TurnState)>, mut parent_alpha: f32, mut parent_beta: f32, layer: usize, maximizing: bool) -> (f32, usize) {
        if layer == 0 || current_node.data.2 != TurnState::Continue {
            return (current_node.data_score, current_node.data.1);
        }
        let possible_moves = self.get_possible_moves(&current_node.data.0, maximizing);
        return if maximizing {
            let mut best_move: (f32, usize) = (f32::NEG_INFINITY, 0);
            for m in possible_moves.iter() {
                let (childs_best, _) = self.compute_next_move(m, parent_alpha, parent_beta, layer - 1, false);
                if best_move.0 < childs_best {
                    best_move = (childs_best, m.data.1)
                }
                parent_alpha = parent_alpha.max(best_move.0);
                if parent_beta <= parent_alpha {
                    break
                }
            }
            best_move
        } else {
            let mut best_move: (f32, usize) = (f32::INFINITY, 0);
            for m in possible_moves.iter() {
                let (childs_best, _) = self.compute_next_move(m, parent_alpha, parent_beta, layer - 1, true);
                if best_move.0 > childs_best {
                    best_move = (childs_best, m.data.1)
                }
                parent_beta = parent_beta.min(best_move.0);
                if parent_beta <= parent_alpha {
                    break
                }
            }
            best_move
        }
    }

    fn sort_moves<T>(moves: &mut Vec<Node<T>>, ascending: bool) where T: PartialEq {
        if ascending {
            moves.sort_unstable_by(|a, b| { a.partial_cmp(b).unwrap() });
        }else {
            moves.sort_unstable_by(|a, b| { b.partial_cmp(a).unwrap() });
        }
    }

    fn get_possible_moves(&self, current_board: &TicTacToe, own_turn: bool) -> Vec<Node<(TicTacToe, usize, TurnState)>>{
        let mut moves = vec![];
        for square in current_board.squares.iter().enumerate() {
            if let SquareState::None = square.1 {
                let move_state: TurnState;
                let square_state;
                if own_turn {
                    square_state = SquareState::Filled(self.symbol);
                }else {
                    square_state = SquareState::Filled(self.op_symbol);
                }
                let mut board_binding = current_board.clone();
                move_state = board_binding.set_square_from_index(square.0, square_state);
                let mut possible_move_node = Node::new((board_binding, square.0, move_state.clone()));
                match move_state {
                    TurnState::Draw => {
                        possible_move_node.data_score = 0.0;
                    }
                    TurnState::Victory => {
                        if own_turn {
                            possible_move_node.data_score = 1.0;
                        }else {
                            possible_move_node.data_score = -1.0;
                        }
                    }
                    TurnState::Continue => {
                        possible_move_node.data_score = self.get_move_heuristic(&possible_move_node.data.0, square_state, square.0, own_turn)
                    }
                    _ => {
                        panic!("Unexpected Ai Error")
                    }
                }
                moves.push(possible_move_node);
            }
        }
        if self.max_moves_to_analyze > 0 && moves.len() > self.max_moves_to_analyze {
            Ai::sort_moves(&mut moves, !own_turn);
            moves = moves.split_at(self.max_moves_to_analyze).0.to_owned(); // cuts the amount of possible moves, keeping only the best ones
        }
        moves.shuffle(&mut thread_rng());
        moves
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
        let attack_score = board.sum_squares_in_winnable_distance(index, square_state, false) as f32;
        let available_axis = board.check_n_of_available_axis(index, square_state) as f32;
        let op_square = if own_turn { SquareState::Filled(self.op_symbol) } else { SquareState::Filled(self.symbol) };
        let mut defense_score = board.sum_squares_in_winnable_distance(index, op_square, false) as f32;
        let mut empty_space_around_score = board.spaces_of_around(index, SquareState::None) as i32;
        let x_size = board.x_size;
        let y_size = board.y_size;
        let coord = board.get_index_coord(index);
        if board.seq_to_win.pow(2) < board.size() {
            if (board.seq_to_win % 2 != 0 && defense_score >= (board.seq_to_win as f32 / 2.0).ceil()) ||
                (board.seq_to_win % 2 == 0 && defense_score >= (board.seq_to_win as f32 - 2.0)) {
                defense_score *= 100.0;
            }
        }
        if coord.1 == 0 || coord.1 == y_size - 1 {
            if coord.0 > 0 && coord.0 < x_size - 1 {
                empty_space_around_score = 0;
            }
        } else if coord.0 == 0 || coord.0 == x_size - 1 {
            empty_space_around_score = 0;
        }
        defense_score /= 10.0;
        let heuristic = ((attack_score + defense_score + (available_axis) / 10.0) + (empty_space_around_score as f32 / 100.0)) / 100.0;
        return if own_turn {
            heuristic
        }else {
            -heuristic
        }
    }
}
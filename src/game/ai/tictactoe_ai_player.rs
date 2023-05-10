use crate::game::ai::tree::{Node, Tree};
use crate::game::tictactoe_core::{SquareState, TicTacToe, TurnState};

#[derive(Clone)]
pub struct Ai{
    max_node_childs: isize,
    pub symbol: char,
    pub op_symbol: char,
    pub debug_mode: bool,
    tree: Tree<(TicTacToe, usize)>
}

impl Ai {
    pub fn create(max_node_childs: isize, symbol: char, op_symbol: char) -> Ai {
        Ai {
            max_node_childs,
            symbol,
            op_symbol,
            debug_mode: false,
            tree: Tree::new(),
        }
    }
    pub fn act(&mut self, current_board: TicTacToe) -> (usize, usize) {

        let mut root = Node::new((current_board.clone(), 0));
        self.load_moves_tree(&mut root, true);
        self.tree.root = Some(root);

        let next_action = self.tree.root.clone().unwrap().minimax(true);
        let coord = current_board.get_index_coord(next_action.data.1);
        return coord
    }

    fn load_moves_tree(&self, parent: &mut Node<(TicTacToe, usize)>, own_turn: bool) {
        for square in parent.data.0.squares.iter().enumerate() {
            if let SquareState::None = square.1 {
                let mut possible_move = parent.data.clone();
                possible_move.1 = square.0;

                let move_state: TurnState;
                if own_turn {
                    move_state = possible_move.0.set_square_from_index(square.0, SquareState::Filled(self.symbol));
                }else {
                    move_state = possible_move.0.set_square_from_index(square.0, SquareState::Filled(self.op_symbol));
                }

                let mut possible_move_node = Node::new(possible_move);
                match move_state {
                    TurnState::Draw => {
                        possible_move_node.utility = Some(0);
                    }
                    TurnState::Victory => {
                        if own_turn {
                            possible_move_node.utility = Some(1);
                        }else {
                            possible_move_node.utility = Some(-1);
                        }
                    }
                    TurnState::Continue => {
                        self.load_moves_tree(&mut possible_move_node, !own_turn);
                    }
                    _ => {
                        panic!("Unexpected Ai Error")
                    }// should not match TurnState::Error
                }
                parent.childs.push(possible_move_node);
            }
        }
        let best_move = parent.minimax(own_turn);
        parent.utility = best_move.utility;
        parent.childs = vec![best_move];
    }
}

#[derive(Clone, PartialEq)]
struct AiBoardView {
    pub vector: Vec<SquareState>,
    x_size: usize,
    y_size: usize,
    pub utility: Option<i32>
}

impl AiBoardView {
    pub fn new(x_size: usize, y_size: usize) -> AiBoardView {
        AiBoardView {
            vector: vec![SquareState::None; x_size*y_size],
            x_size,
            y_size,
            utility: None,
        }
    }
    pub fn from(game: TicTacToe) -> AiBoardView {
        let mut board = AiBoardView {
            vector: vec![SquareState::None; game.x_size*game.y_size],
            x_size: game.x_size,
            y_size: game.y_size,
            utility: None,
        };
        for i in 0..game.squares.len() {
            board.vector[i] = game.squares.get(i).unwrap().clone();
        }
        board
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&SquareState> {
        self.vector.get(x + self.x_size*y)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut SquareState> {
        self.vector.get_mut(x + self.x_size*y)
    }
    pub fn is_empty(&self) -> bool {
        for i in self.vector.iter() {
            if let SquareState::None = i {
                continue
            } else {
                return false;
            }
        }
        true
    }
}
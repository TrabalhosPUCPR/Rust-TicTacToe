use crate::game::ai::tree::{Node, Tree, Utility};
use crate::game::tictactoe_core::{SquareState, TicTacToe};

#[derive(Clone)]
pub struct Ai{
    max_node_childs: isize,
    max_layers: isize,
    tree: Tree<AiBoardView>,
    pub symbol: char,
    pub op_symbol: char
}

impl Ai {
    pub fn create(max_node_childs: isize, max_layers: isize, symbol: char, op_symbol: char) -> Ai {
        Ai {
            max_node_childs,
            max_layers,
            tree: Tree::new(),
            symbol,
            op_symbol,
        }
    }
    pub fn act(&mut self, current_board: TicTacToe) -> (usize, usize) {
        let current_view = AiBoardView::from(current_board.clone());
        if let Some(root) = &self.tree.root {
            // read from previously created tree
            for child in root.childs.clone() {
                if child.data == current_view {
                    self.tree.set_root(child.data);
                    break
                }
            }
        }else {
            // loads new tree
            let mut initial = Node::new(AiBoardView::from(current_board));
            self.add_all_possible_moves(&mut initial, true);
            self.tree.root = Some(initial);
        }
        // now decides were it should go
        todo!()
    }
    fn add_all_possible_moves(&self, parent: &mut Node<AiBoardView>, own_turn: bool) {
        let parent_values = parent.clone();
        for square in parent_values.data.vector.iter().enumerate() {
            if let SquareState::None = square.1 {
                let mut possible_move = parent.data.clone();

                if own_turn {
                    possible_move.vector[square.0] = SquareState::Filled(self.symbol);
                }else {
                    possible_move.vector[square.0] = SquareState::Filled(self.op_symbol);
                }

                let mut possible_move_node = Node::new(possible_move);
                self.add_all_possible_moves(&mut possible_move_node, !own_turn);

                parent.childs.push(possible_move_node);
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct AiBoardView {
    pub vector: Vec<SquareState>,
    x_size: usize,
    y_size: usize,
    pub utility: Option<i32>
}

impl Utility for AiBoardView {
    fn get_utility(&self) -> i32 {
        if let Some(x) = self.utility {
            return x
        }
        0
    }
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
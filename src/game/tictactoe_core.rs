use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub enum SquareState {
    Filled(char),
    None
}

pub enum TurnState {
    Continue,
    Draw,
    Error,
    Victory
}

#[derive(Clone)]
pub struct TicTacToe {
    pub x_size: usize,
    pub y_size: usize,
    horizontal_spacer: char,
    vertical_spacer: char,
    pub squares: Vec<SquareState>,
    pub seq_to_win: usize,
    pub empty_space_symbol: char,
    filled: usize,
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            x_size: 4,
            y_size: 4,
            horizontal_spacer: '-',
            vertical_spacer: '|',
            squares: vec![SquareState::None; 16],
            seq_to_win: 4,
            empty_space_symbol: ' ',
            filled: 0
        }
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut lane = 0;
        for i in 0..self.squares.len() {
            match self.squares.get(i).unwrap() {
                SquareState::Filled(c) => {
                    s.push_str(&*format!(" {} ", c))
                }
                SquareState::None => {
                    s.push_str(&*format!(" {} ", self.empty_space_symbol))
                }
            }
            if (i > 0) && ((i + 1) % self.x_size == 0) && !(lane + 1 == self.y_size) {
                s.push_str("\n");
                for _ in 0..(self.x_size*3)+self.x_size-1 {
                    s.push(self.horizontal_spacer)
                }
                s.push_str("\n");
                lane += 1
            }else if i < self.squares.len() - 1 {
                s.push(self.vertical_spacer)
            }
        }
        write!(f, "{}", s)
    }
}

impl TicTacToe {
    pub fn new(x_size: usize, y_size: usize, seq_to_win: usize, empty_space_symbol: char) -> TicTacToe {
        TicTacToe {
            x_size,
            y_size,
            horizontal_spacer: '-',
            vertical_spacer: '|',
            squares: vec![SquareState::None; x_size*y_size],
            seq_to_win,
            empty_space_symbol,
            filled: 0
        }
    }

    pub fn get_square(&self, x: usize, y: usize) -> Option<&SquareState> {
        let i = self.get_coord_index(x, y);
        self.squares.get(i)
    }

    fn get_coord_index(&self, x: usize, y: usize) -> usize {
        x + self.x_size*y
    }

    pub fn set_square(&mut self, x: usize, y: usize, state: SquareState) -> TurnState {
        let i = self.get_coord_index(x, y);
        if self.squares[i] != SquareState::None {
            return TurnState::Error
        }
        self.squares[i] = state;
        self.filled += 1;
        self.check_game_over(x, y, state)
    }

    fn check_game_over(&mut self, x: usize, y: usize, state: SquareState) -> TurnState {
        if self.check_draw() {
            return TurnState::Draw
        }
        if (self.check_x(x, y, state) >= self.seq_to_win) ||
            (self.check_y(x, y, state) == self.seq_to_win) ||
            (self.check_left_diag(x, y, state) == self.seq_to_win) ||
            (self.check_right_diag(x, y, state) == self.seq_to_win) {
            return TurnState::Victory
        }
        return TurnState::Continue
    }

    fn check_y(&self, x: usize, y: usize, state: SquareState) -> usize {
        let mut seq_count = 1;
        let mut y_check = y.checked_sub(1); // starts looking above, checked sub is to avoid going below 0
        while y_check.is_some() && (self.get_square(x, y_check.unwrap()).unwrap().clone() == state) {
            seq_count += 1;
            y_check = y_check.unwrap().checked_sub(1);
        }
        let mut y_check = y + 1; // then looks below
        while (y_check < self.x_size) && (self.get_square(x, y_check).unwrap().clone() == state) {
            seq_count += 1;
            y_check += 1;
        }
        seq_count
    }

    fn check_x(&self, x: usize, y: usize, state: SquareState) -> usize {
        let mut seq_count = 1;
        let mut x_check = x.checked_sub(1); // starts looking to the left
        while x_check.is_some() && (self.get_square(x_check.unwrap(), y).unwrap().clone() == state) {
            seq_count += 1;
            x_check = x_check.unwrap().checked_sub(1);
        }
        let mut x_check = x + 1; // then looks to the right
        while (x_check < self.x_size) && (self.get_square(x_check, y).unwrap().clone() == state) {
            seq_count += 1;
            x_check += 1;
        }
        seq_count
    }

    fn check_left_diag(&self, x: usize, y: usize, state: SquareState) -> usize { // \
        let mut seq_count = 1;

        let mut x_check = x.checked_sub(1); // checks left diag
        let mut y_check = y.checked_sub(1);

        while x_check.is_some() &&
            y_check.is_some() &&
            (self.get_square(x_check.unwrap(), y_check.unwrap()).unwrap().clone() == state) {
            seq_count += 1;
            x_check = x_check.unwrap().checked_sub(1);
            y_check = y_check.unwrap().checked_sub(1);
        }

        let mut x_check = x + 1;
        let mut y_check = y + 1;

        while (x_check < self.x_size) &&
            (y_check < self.y_size) &&
            (self.get_square(x_check, y_check).unwrap().clone() == state) {
            seq_count += 1;
            x_check += 1;
            y_check += 1;
        }
        seq_count
    }

    fn check_right_diag(&self, x: usize, y: usize, state: SquareState) -> usize { // /
        let mut seq_count = 1;

        let mut x_check = x.checked_sub(1);
        let mut y_check = y + 1;

        while x_check.is_some() &&
            (y_check < self.y_size) &&
            (self.get_square(x_check.unwrap(), y_check).unwrap().clone() == state) {
            seq_count += 1;
            x_check = x_check.unwrap().checked_sub(1);
            y_check += 1;
        }

        let mut x_check = x + 1;
        let mut y_check = y.checked_sub(1);

        while (x_check < self.x_size) &&
            y_check.is_some() &&
            (self.get_square(x_check, y_check.unwrap()).unwrap().clone() == state) {
            seq_count += 1;
            x_check += 1;
            y_check = y_check.unwrap().checked_sub(1);
        }
        seq_count
    }

    fn check_draw(&self) -> bool {
        self.filled == self.squares.len()
    }

    pub fn size(&self) -> usize {
        self.x_size * self.y_size
    }

    pub fn get_by_index(&self, index: usize) -> Option<&SquareState> {
        self.squares.get(index)
    }
}

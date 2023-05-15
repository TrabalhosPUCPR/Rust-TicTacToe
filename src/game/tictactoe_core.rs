use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub enum SquareState {
    Filled(char),
    None
}

#[derive(Clone, PartialEq)]
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
    pub filled: usize,
}

impl PartialEq for TicTacToe {
    fn eq(&self, other: &Self) -> bool {
        self.squares.eq(&other.squares)
    }

    fn ne(&self, other: &Self) -> bool {
        self.squares.ne(&other.squares)
    }
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            x_size: 3,
            y_size: 3,
            horizontal_spacer: '-',
            vertical_spacer: '|',
            squares: vec![SquareState::None; 9],
            seq_to_win: 3,
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
        if x >= self.x_size || y >= self.y_size {
            return None
        }
        let i = self.get_coord_index(x, y);
        self.squares.get(i)
    }

    pub fn get_coord_index(&self, x: usize, y: usize) -> usize {
        x + self.x_size*y
    }
    pub fn get_index_coord(&self, i: usize) -> (usize, usize) {
        let x = i % self.x_size;
        let y = i / self.x_size;
        (x, y)
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

    pub fn set_square_from_index(&mut self, i: usize, state: SquareState) -> TurnState {
        if self.squares[i] != SquareState::None {
            return TurnState::Error
        }
        self.squares[i] = state;
        self.filled += 1;
        let coord = self.get_index_coord(i);
        self.check_game_over(coord.0, coord.1, state)
    }

    fn check_game_over(&mut self, x: usize, y: usize, state: SquareState) -> TurnState {
        if self.filled >= self.seq_to_win + 2 &&
            ((self.check_x(x, y, state, true) >= self.seq_to_win) ||
                (self.check_y(x, y, state, true) == self.seq_to_win) ||
                (self.check_left_diag(x, y, state, true) == self.seq_to_win) ||
                (self.check_right_diag(x, y, state, true) == self.seq_to_win))  {
            return TurnState::Victory
        }
        if self.check_draw() {
            return TurnState::Draw
        }
        return TurnState::Continue
    }

    pub fn sum_of_same_squares_in_winnable_distance(&self, index: usize, state: SquareState) -> i32 {
        let coord = self.get_index_coord(index);
        ((self.check_x(coord.0, coord.1, state, false) +
            self.check_y(coord.0, coord.1, state, false) +
            self.check_left_diag(coord.0, coord.1, state, false) +
            self.check_right_diag(coord.0, coord.1, state, false)) - 4) as i32

    }

    pub fn empty_spaces_around(&self, index: usize) -> usize {
        let coord = self.get_index_coord(index);
        let mut count = 0;
        if let Some(n) = coord.0.checked_sub(1) {
            if let Some(s) = self.get_square(n, coord.1) {
                if let SquareState::None = s {
                    count += 1
                }
            }
        }
        if let Some(s) = self.get_square(coord.0 + 1, coord.1) {
            if let SquareState::None = s {
                count += 1
            }
        }
        let mut x = 0;
        if coord.0 != 0 {
            x = coord.0 - 1
        }
        for i in 0..3 {
            if let Some(i) = coord.1.checked_sub(1) {
                if let Some(s) = self.get_square(x + i, coord.1 - 1) {
                    if let SquareState::None = s {
                        count += 1
                    }
                }else {
                    break
                }
            }
            if let Some(s) = self.get_square(x + i, coord.1 + 1) {
                if let SquareState::None = s {
                    count += 1
                }
            }
        }
        count as usize
    }

    fn check_y(&self, x: usize, y: usize, state: SquareState, stop_counting: bool) -> usize {
        //let mut available_spaces_count = 1;
        let mut seq_count = 1;
        let mut dist = 1;
        let mut y_check = y.checked_sub(1); // starts looking above, checked sub is to avoid going below 0
        while dist <= self.seq_to_win && y_check.is_some() {
            let square_state = self.get_square(x, y_check.unwrap()).unwrap().clone();
            if square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None {
                break
            }
            dist += 1;
            y_check = y_check.unwrap().checked_sub(1);
        }
        let mut y_check = y + 1; // then looks below
        dist = 1;
        while dist < self.seq_to_win && y_check < self.x_size {
            let square_state = self.get_square(x, y_check).unwrap().clone();
            if square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None {
                break
            }
            dist += 1;
            y_check += 1;
        }
        seq_count
    }

    fn check_x(&self, x: usize, y: usize, state: SquareState, stop_counting: bool) -> usize {
        let mut seq_count = 1;
        let mut x_check = x.checked_sub(1); // starts looking to the left
        let mut dist = 1;
        while dist <= self.seq_to_win && x_check.is_some() {
            let square_state = self.get_square(x_check.unwrap(), y).unwrap().clone();
            if square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None { break }
            dist += 1;
            x_check = x_check.unwrap().checked_sub(1);
        }
        let mut x_check = x + 1; // then looks to the right
        dist = 1;
        while dist <= self.seq_to_win && x_check < self.x_size {
            let square_state = self.get_square(x_check, y).unwrap().clone();
            if  square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None { break }
            dist += 1;
            x_check += 1;
        }
        seq_count
    }

    fn check_left_diag(&self, x: usize, y: usize, state: SquareState, stop_counting: bool) -> usize { // \
        let mut seq_count = 1;

        let mut x_check = x.checked_sub(1); // checks left diag
        let mut y_check = y.checked_sub(1);

        while x_check.is_some() && y_check.is_some() {
            let square_state = self.get_square(x_check.unwrap(), y_check.unwrap()).unwrap().clone();
            if square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None { break }
            x_check = x_check.unwrap().checked_sub(1);
            y_check = y_check.unwrap().checked_sub(1);
        }

        let mut x_check = x + 1;
        let mut y_check = y + 1;
        while (x_check < self.x_size) &&
            (y_check < self.y_size) {
            let square_state = self.get_square(x_check, y_check).unwrap().clone();
            if square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None { break }
            x_check += 1;
            y_check += 1;
        }
        seq_count
    }

    fn check_right_diag(&self, x: usize, y: usize, state: SquareState, stop_counting: bool) -> usize { // /
        let mut seq_count = 1;

        let mut x_check = x.checked_sub(1);
        let mut y_check = y + 1;

        while x_check.is_some() && y_check < self.y_size {
            let square_state = self.get_square(x_check.unwrap(), y_check).unwrap().clone();
            if square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None { break }
            x_check = x_check.unwrap().checked_sub(1);
            y_check += 1;
        }

        let mut x_check = x + 1;
        let mut y_check = y.checked_sub(1);

        while (x_check < self.x_size) &&
            y_check.is_some() {
            let square_state = self.get_square(x_check, y_check.unwrap()).unwrap().clone();
            if square_state == state {
                seq_count += 1;
            }else if stop_counting || square_state != SquareState::None { break }
            x_check += 1;
            y_check = y_check.unwrap().checked_sub(1);
        }
        seq_count
    }

    pub fn check_draw(&self) -> bool {
        self.filled == self.squares.len()
    }

    pub fn size(&self) -> usize {
        self.x_size * self.y_size
    }

    pub fn get_square_by_index(&self, index: usize) -> Option<&SquareState> {
        self.squares.get(index)
    }

    pub fn clear(&mut self) {
        self.squares = vec![SquareState::None; self.x_size*self.y_size];
        self.filled = 0
    }
}

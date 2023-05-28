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
    cross_spacer: char,
    pub squares: Vec<SquareState>,
    pub seq_to_win: usize,
    pub empty_space_symbol: char,
    pub filled: usize,
}

impl Display for TurnState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            TurnState::Continue => write!(f, "Continue"),
            TurnState::Draw => write!(f, "Draw"),
            TurnState::Error => write!(f, "Error"),
            TurnState::Victory => write!(f, "Victory"),
        }
    }
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
            horizontal_spacer: '─',
            vertical_spacer: '│',
            cross_spacer: '┼',
            squares: vec![SquareState::None; 9],
            seq_to_win: 3,
            empty_space_symbol: ' ',
            filled: 0
        }
    }
}

pub trait SpecialBoardChecks {
    fn sum_squares_in_winnable_distance(&self, index: usize, state: SquareState, return_highest: bool) -> i32;
    fn spaces_of_around(&self, index: usize, state: SquareState) -> usize;
    fn check_n_of_available_axis(&self, index: usize, state: SquareState) -> usize;
    fn check_blocked_op_spaces(&self, index: usize, op_placed_square: SquareState) -> [usize; 4];
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn add_col_margin(s: &mut String, col_margin: usize){
            for _ in 0..col_margin {
                s.push_str(" ")
            }
        }
        fn add_line_margin(s: &mut String, line_margin: usize){
            for _ in 1..line_margin {
                s.push_str(" ")
            }
        }
        let mut s = String::new();
        let mut lane = 1;
        let mut col_margin = self.y_size.to_string().len()+4;
        if col_margin % 2 == 0 {
            col_margin += 1;
        }
        let mut line_margin = self.x_size.to_string().len();
        let mut col = 1;
        add_col_margin(&mut s, col_margin/2);
        for i in 0..(self.x_size*3) {
            if i % 3 == 0 {
                //add_col_margin(&mut s, (col_margin/2)-1);
                s.push_str(format!(" {}", col).as_str());
                col += 1
            }else {
                //add_col_margin(&mut s, (col_margin/2)-1);
                s.push_str(" ");
            }
        }
        s.push_str(format!("\n{}-", lane).as_str());
        add_line_margin(&mut s, line_margin);
        for i in 0..self.squares.len() {
            match self.squares.get(i).unwrap() {
                SquareState::Filled(c) => {
                    //add_col_margin(&mut s, col_margin/2);
                    s.push_str(&*format!(" {} ", c));
                }
                SquareState::None => {
                    //add_col_margin(&mut s, col_margin/2);
                    s.push_str(&*format!(" {} ", self.empty_space_symbol));
                }
            }
            if (i > 0) && ((i + 1) % self.x_size == 0) && !(lane == self.y_size) {
                s.push_str("\n");
                add_col_margin(&mut s, col_margin/2);
                for i in 0..(self.x_size*3)+self.x_size-1 {
                    if (i + 1) % 4 == 0 {
                        s.push(self.cross_spacer)
                    }else {
                        s.push(self.horizontal_spacer)
                    }
                }
                lane += 1;
                s.push_str(format!("\n{}-", lane).as_str());
                add_line_margin(&mut s, line_margin);
                if (lane+1).to_string().len() > lane.to_string().len() {
                    line_margin -= 1
                }
            }else if i < self.squares.len() - 1 {
                s.push(self.vertical_spacer)
            }
        }
        write!(f, "{}\n", s)
    }
}

impl SpecialBoardChecks for TicTacToe {
    fn sum_squares_in_winnable_distance(&self, index: usize, state: SquareState, return_highest: bool) -> i32 {
        let coord = self.get_index_coord(index);
        return if return_highest {
            let mut count: [usize; 4] = [0,0,0,0];
            count[0] = self.check_x(coord.0, coord.1, state, false, false) - 1;
            count[1] = self.check_y(coord.0, coord.1, state, false, false) - 1;
            count[2] = self.check_left_diag(coord.0, coord.1, state, false, false) - 1;
            count[3] = self.check_right_diag(coord.0, coord.1, state, false, false) - 1;
            let mut highest = count[0];
            for i in 1..count.len() {
                if count[i] > highest {
                    highest = count[i]
                }
            }
            highest as i32
        }else {
            ((self.check_x(coord.0, coord.1, state, false, false) +
                self.check_y(coord.0, coord.1, state, false, false) +
                self.check_left_diag(coord.0, coord.1, state, false, false) +
                self.check_right_diag(coord.0, coord.1, state, false, false)) - 4) as i32
        }
    }

    fn spaces_of_around(&self, index: usize, state: SquareState) -> usize {
        let coord = self.get_index_coord(index);
        let mut count = 0;
        if let Some(n) = coord.0.checked_sub(1) {
            if let Some(s) = self.get_square(n, coord.1) {
                if state == s.clone() {
                    count += 1
                }
            }
        }
        if let Some(s) = self.get_square(coord.0 + 1, coord.1) {
            if state == s.clone() {
                count += 1
            }
        }
        let mut x = 0;
        let mut n = 2;
        if coord.0 != 0 {
            x = coord.0 - 1;
            n = 3;
        }
        let top_y =  coord.1.checked_sub(1);
        for i in 0..n {
            if let Some(y) = top_y {
                if let Some(s) = self.get_square(x + i, y) {
                    if state == s.clone() {
                        count += 1
                    }
                }
            }
            if let Some(s) = self.get_square(x + i, coord.1 + 1) {
                if state == s.clone() {
                    count += 1
                }
            }
        }
        count as usize
    }

    fn check_n_of_available_axis(&self, index: usize, state: SquareState) -> usize {
        let coord = self.get_index_coord(index);
        let mut axis = 0;
        if self.check_x(coord.0, coord.1, state, false, true) >= self.seq_to_win {
            axis += 1
        }
        if self.check_y(coord.0, coord.1, state, false, true) >= self.seq_to_win {
            axis += 1
        }
        if self.check_left_diag(coord.0, coord.1, state, false, true) >= self.seq_to_win {
            axis += 1
        }
        if self.check_right_diag(coord.0, coord.1, state, false, true) >= self.seq_to_win {
            axis += 1
        }
        axis
    }

    fn check_blocked_op_spaces(&self, index: usize, op_placed_square: SquareState) -> [usize; 4] {
        let mut count: [usize; 4] = [0,0,0,0];
        let coord = self.get_index_coord(index);

        count[0] = self.check_x(coord.0, coord.1, op_placed_square, false, false) - 1;
        count[1] = self.check_y(coord.0, coord.1, op_placed_square, false, false) - 1;
        count[2] = self.check_left_diag(coord.0, coord.1, op_placed_square, false, false) - 1;
        count[3] = self.check_right_diag(coord.0, coord.1, op_placed_square, false, false) - 1;

        count
    }
}

impl TicTacToe {
    pub fn new(x_size: usize, y_size: usize, seq_to_win: usize, empty_space_symbol: char) -> TicTacToe {
        let mut t = TicTacToe::default();
        t.empty_space_symbol = empty_space_symbol;
        t.y_size = y_size;
        t.x_size = x_size;
        t.seq_to_win = seq_to_win;
        t.squares = vec![SquareState::None; x_size*y_size];
        t
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
            ((self.check_x(x, y, state, true, false) >= self.seq_to_win) ||
                (self.check_y(x, y, state, true, false) == self.seq_to_win) ||
                (self.check_left_diag(x, y, state, true, false) == self.seq_to_win) ||
                (self.check_right_diag(x, y, state, true, false) == self.seq_to_win))  {
            return TurnState::Victory
        }
        if self.check_draw() {
            return TurnState::Draw
        }
        return TurnState::Continue
    }

    fn all_lines_checker(&self, x: usize, y: usize, state: &SquareState, stop_counting: bool, check_x_axis: bool, check_y_axis: bool, inverted: bool) -> (usize, usize) {
        let mut available_spaces_count = 1;
        let mut seq_count = 1;
        let mut dist;
        let loop_condition: Box<dyn Fn(i32, (&mut usize, &mut usize)) -> bool> = if check_x_axis == check_y_axis {
            Box::new(|dist, (check_x, check_y)| {
                let (offset_x, offset_y) = if inverted { (dist, -dist) } else { (dist, dist) };
                let result = x as i32 + offset_x >= 0 && y as i32 + offset_y >= 0 && dist < self.seq_to_win as i32;
                if result {
                    *check_y = (y as i32 + offset_y) as usize;
                    *check_x = (x as i32 + offset_x) as usize;
                }
                result
            })
        } else {
            Box::new(|dist, (check_x, check_y)| {
                let (value, to_check) = if check_x_axis {
                    (x, check_x)
                } else {
                    (y, check_y)
                };
                let result = value as i32 + dist >= 0 && dist < self.seq_to_win as i32;
                if result {
                    *to_check = (value as i32 + dist) as usize;
                }
                result
            })
        };
        for i in [1, -1].iter().copied() {
            dist = i;
            let mut checking_x = x;
            let mut checking_y = y;
            while loop_condition(dist, (&mut checking_x, &mut checking_y)) {
                let square_state = self.get_square(checking_x, checking_y);
                if let Some(n) = square_state {
                    if n == state {
                        seq_count += 1;
                    }else if stop_counting || n != &SquareState::None {
                        break
                    }
                }else {
                    break
                }
                dist += i;
                available_spaces_count += 1;
            }
        }
        (seq_count, available_spaces_count)
    }

    fn check_for_victory(&self, x: usize, y: usize, state: SquareState, stop_counting: bool, return_available_spaces: bool, x_axis: bool, y_axis: bool, inverted: bool) -> usize { // |
        let result = self.all_lines_checker(x, y, &state, stop_counting, x_axis, y_axis, inverted);
        return if return_available_spaces {
            result.1
        }else if !stop_counting || result.0 >= self.seq_to_win {
            result.0
        }else {
            1
        }
    }

    fn check_x(&self, x: usize, y: usize, state: SquareState, stop_counting: bool, return_available_spaces: bool) -> usize { // -
        return self.check_for_victory(x, y, state, stop_counting, return_available_spaces, true, false, false)
    }
    fn check_y(&self, x: usize, y: usize, state: SquareState, stop_counting: bool, return_available_spaces: bool) -> usize { // |
        return self.check_for_victory(x, y, state, stop_counting, return_available_spaces, false, true, false)
    }
    fn check_left_diag(&self, x: usize, y: usize, state: SquareState, stop_counting: bool, return_available_spaces: bool) -> usize { // \
        return self.check_for_victory(x, y, state, stop_counting, return_available_spaces, true, true, false)
    }
    fn check_right_diag(&self, x: usize, y: usize, state: SquareState, stop_counting: bool, return_available_spaces: bool) -> usize { // /
        return self.check_for_victory(x, y, state, stop_counting, return_available_spaces, true, true, true)
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

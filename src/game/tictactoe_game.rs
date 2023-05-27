use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};
use std::string::ToString;
use crate::game::ai::tictactoe_ai_player::Ai;
use crate::game::tictactoe_core::{SquareState, TicTacToe, TurnState};
use crate::game::turn_logger::TurnLogger;

pub struct TicTacToeGame {
    board: TicTacToe,
    player1: Player,
    player2: Player,
    game_state: GameState,
    pub show_turn_info_mode: bool
}

#[derive(Clone)]
pub enum PlayerType {
    Human,
    Computer(Ai)
}

#[derive(Clone)]
pub struct Player {
    p_type: PlayerType,
    name: String,
    square_symbol: char
}

enum GameState {
    Player(usize, Player), // number, player info
    Finished,
    Begin(usize)
}

pub enum AiDifficulties {
    Easy,
    Medium,
    Hard
}

impl Display for TicTacToeGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let state = &self.game_state;
        return match state {
            GameState::Player(_, s) => {
                write!(f, "\n{}'s turn:\n\n", s.name).and(self.board.fmt(f))
            }
            GameState::Finished => {
                write!(f, "Game is finished!\n\n").and(self.board.fmt(f))
            }
            GameState::Begin(_) => {
                write!(f, "Game have not started!\n\n").and(self.board.fmt(f))
            }
        }
    }
}

const TITLE: &str = "
  _______     _______      _______
 |__   __|   |__   __|    |__   __|
    | |   _  ___| | __ _  ___| | ___   ___
    | |  | |/ __| |/ _` |/ __| |/ _ \\ / _ \\
    | |  | | (__| | (_| | (__| | (_) |  __/
    |_|  |_|\\___|_|\\__,_|\\___|_|\\___/ \\___|";
const AUTHOR: &str = "KnightLeo";
const REPO_LINK: &str = "https://github.com/TrabalhosPUCPR/Rust-TicTacToe";


impl TicTacToeGame {
    pub fn start_new_game_prompts() {
        loop {
            println!("{}\t\tMade by {}\nRepo link: {}\n\nPlease choose an option:\n1-Load 1 player game\n2-Load 2 player game\n(default: 1)\n", TITLE, AUTHOR, REPO_LINK);
            let mut game;
            let n = TicTacToeGame::input_usize();
            if n.is_none() || n.unwrap() == 1 {
                println!("\nSelect a difficulty for the AI\n1-Easy\n2-Medium\n3-Hard\n(default: 3)\n");
                let mut difficulty = AiDifficulties::Hard;
                match TicTacToeGame::input_usize() {
                    None => {}
                    Some(n) => {
                        match n {
                            1 => difficulty = AiDifficulties::Easy,
                            2 => difficulty = AiDifficulties::Medium,
                            _ => {}
                        }
                    }
                }
                game = TicTacToeGame::load_default_1player_game(difficulty)
            }else if n.unwrap() == 2 {
                game = TicTacToeGame::load_default_2player_game()
            }else {
                println!("Type a valid number!");
                continue
            }
            println!("Board Size: {}\nSequence to win: {}\nPlayer 1 symbol: {}\nPlayer 2 symbol: {}", game.board.size(), game.board.seq_to_win, game.player1.square_symbol, game.player2.square_symbol);
            println!("\n1-Start Game\n2-Configure Game\n(default: 1)");
            let result = TicTacToeGame::input_usize();
            if result.is_some() && result.unwrap() == 2 {
                loop {
                    println!("\nType board size:");
                    let size;
                    if let Some(n) = TicTacToeGame::input_usize() {
                        size = n;
                    }else {
                        println!("Invalid Input");
                        continue
                    }
                    println!("\nType sequence length to win:");
                    let length;
                    if let Some(n) = TicTacToeGame::input_usize() {
                        length = n;
                    }else {
                        println!("Invalid Input");
                        continue
                    }
                    if !game.change_size(size, length) {
                        println!("Board size and sequence to win are not valid");
                        continue
                    }
                    println!("Board Size: {}\nSequence to win: {}\nPlayer 1 symbol: {}\nPlayer 2 symbol: {}", game.board.size(), game.board.seq_to_win, game.player1.square_symbol, game.player2.square_symbol);
                    break
                }
            }
            loop {
                game.start_game();
                println!("Would you like to reload? (Y/n)\n");
                if !TicTacToeGame::input_bool() {
                    break
                }
                game.reload_game()
            }
        }
    }
    fn input_usize() -> Option<usize> {
        let mut ans: String = String::new();
        stdin().read_line(&mut ans).unwrap();
        ans.remove(ans.len()-1);
        let result: Result<usize, _> = ans.parse();
        return if result.is_ok() {
            Some(result.unwrap())
        }else {
            None
        }
    }
    fn input_bool() -> bool {
        let mut ans: String = String::new();
        stdin().read_line(&mut ans).unwrap();
        ans.remove(ans.len()-1);
        let result: Result<char, _> = ans.parse();
        if result.is_ok() && result.unwrap() == 'n' {
            return false
        }
        true
    }
    pub fn load_default_1player_game(ai_difficulty: AiDifficulties) -> TicTacToeGame {
        let mut g: TicTacToeGame = Default::default();
        g.player2.p_type = PlayerType::Computer(TicTacToeGame::create_ai(ai_difficulty, g.player2.square_symbol.clone(), g.player1.square_symbol.clone()));
        return g
    }
    pub fn load_default_2player_game() -> TicTacToeGame {
        Default::default()
    }
    pub fn load_default_ai_game(ai1_difficulty: AiDifficulties, ai2_difficulty: AiDifficulties) -> TicTacToeGame {
        let mut g: TicTacToeGame = TicTacToeGame::load_default_1player_game(ai2_difficulty);
        g.player1.p_type = PlayerType::Computer(TicTacToeGame::create_ai(ai1_difficulty, g.player1.square_symbol.clone(), g.player2.square_symbol.clone()));
        return g
    }
    
    fn create_ai(difficulty: AiDifficulties, symbol: char, op_symbol: char) -> Ai {
        let max_childs: usize; // gets incremented every turn
        let max_layers: usize;
        match difficulty { // 0 == infinite
            AiDifficulties::Easy => {
                max_childs = 7;
                max_layers = 1;
            }
            AiDifficulties::Medium => {
                max_childs = 6;
                max_layers = 2;
            }
            AiDifficulties::Hard => {
                max_childs = 8;
                max_layers = 6;
            }
        }
        Ai::create(max_childs, max_layers, symbol, op_symbol)
    }
    pub fn start_game(&mut self) {
        let mut turn_logger= TurnLogger::start();
        loop {
            match &mut self.game_state {
                GameState::Begin(p) => {
                    match p {
                        1 => self.game_state = GameState::Player(p.to_owned(), self.player1.clone()),
                        2 => self.game_state = GameState::Player(p.to_owned(), self.player2.clone()),
                        _ => {}
                    }
                    println!("{}", self);
                }
                GameState::Player(n, p) => {
                    let col: usize;
                    let line: usize;
                    turn_logger.restart_timer();
                    match &mut p.p_type {
                        PlayerType::Human => {
                            let col_input;
                            let line_input;
                            println!("{}'s turn, type the column of your next move\ncolumn: ", p.name);
                            let mut ans: String = String::new();
                            stdout().flush().expect("");
                            stdin().read_line(&mut ans).unwrap();
                            ans.remove(ans.len()-1);
                            col_input = ans.parse();
                            println!("\nline: ");
                            ans.clear();
                            stdout().flush().expect("");
                            stdin().read_line(&mut ans).unwrap();
                            ans.remove(ans.len()-1);
                            line_input = ans.parse();
                            if col_input.is_ok() && line_input.is_ok() {
                                col = col_input.unwrap();
                                line = line_input.unwrap();
                                if col < 1 || col > self.board.x_size || line < 1 || line > self.board.y_size {
                                    println!("Invalid column or line number");
                                    continue
                                }
                            }else {
                                println!("Not a valid number!");
                                continue
                            }
                        }
                        PlayerType::Computer(ai) => {
                            let ai_action = ai.act(self.board.clone());
                            col = ai_action.0 + 1;
                            line = ai_action.1 + 1;
                        }
                    }
                    turn_logger.end_timer();
                    turn_logger.total_turns += 1;
                    turn_logger.latest_placed_coord = (col, line);
                    turn_logger.player_n_turn = n.clone();
                    let board_state = self.board.set_square(col - 1, line - 1, SquareState::Filled(p.square_symbol));
                    match board_state {
                        TurnState::Draw => {
                            println!("\nAll spaces have been filled! It's a draw!");
                            self.game_state = GameState::Finished
                        }
                        TurnState::Error => {
                            println!("\nType a valid position!")
                        }
                        TurnState::Victory => {
                            println!("\n{} in a row! {} wins!", self.board.seq_to_win, p.name);
                            self.game_state = GameState::Finished
                        }
                        _ => {
                            if n.to_owned().eq(&1) {
                                self.set_current_player_to_2()
                            }else {
                                self.set_current_player_to_1()
                            }
                            println!("{}", self);
                        }
                    }
                    turn_logger.game_state = board_state;
                    if self.show_turn_info_mode {
                        println!("{}", turn_logger)
                    }
                }
                GameState::Finished => {
                    println!("\n{}", self.board);
                    println!("\nGame is finished!");
                    return;
                }
            }
        }
    }
    fn set_current_player_to_1(&mut self) {
        self.game_state = GameState::Player(1, self.player1.clone());
    }
    fn set_current_player_to_2(&mut self) {
        self.game_state = GameState::Player(2, self.player2.clone());
    }
    pub fn set_empty_space_symbol(&mut self, symbol: char) {
        self.board.empty_space_symbol = symbol;
    }
    pub fn set_player1_symbol(&mut self, symbol: char) {
        self.player1.square_symbol = symbol;
        if let PlayerType::Computer(ai) = &mut self.player1.p_type {
            ai.symbol = symbol
        }
        if let PlayerType::Computer(ai) = &mut self.player2.p_type {
            ai.op_symbol = symbol
        }
    }
    pub fn set_player2_symbol(&mut self, symbol: char) {
        self.player2.square_symbol = symbol;
        if let PlayerType::Computer(ai) = &mut self.player2.p_type {
            ai.symbol = symbol
        }
        if let PlayerType::Computer(ai) = &mut self.player1.p_type {
            ai.op_symbol = symbol
        }
    }
    
    pub fn set_first_player(&mut self, player_n: usize){
        if player_n > 0 && player_n < 3 {
            if let GameState::Begin(_) = self.game_state {
                self.game_state = GameState::Begin(player_n)
            }else {
                panic!("Cannot change player during the game!")
            }
        }else {
            panic!("{} Is not a valid player number! Use 1 or 2.", player_n)
        }
    }

    pub fn set_player(&mut self, player_n: usize, player: Player) {
        match player_n {
            1 => {
                self.player1 = player;
            }
            2 => {
                self.player2 = player
            }
            _ => {
                panic!("{} is not a valid player number!", player_n)
            }
        }
        if let PlayerType::Computer(ai) = &mut self.player1.p_type {
            ai.op_symbol = self.player2.square_symbol;
            ai.symbol = self.player1.square_symbol
        }
        if let PlayerType::Computer(ai) = &mut self.player2.p_type {
            ai.op_symbol = self.player1.square_symbol;
            ai.symbol = self.player2.square_symbol
        }
    }

    pub fn change_size(&mut self, size: usize, in_a_row_to_win: usize) -> bool{
        if size < in_a_row_to_win {
            //panic!("Board size of {} is too small to make {} in a row!", size, in_a_row_to_win)
            return false
        }
        self.board.x_size = size;
        self.board.y_size = size;
        self.board.seq_to_win = in_a_row_to_win;
        self.board.squares = vec![SquareState::None; size.pow(2)];
        true
    }

    pub fn reload_game(&mut self) {
        self.game_state = GameState::Begin(1);
        self.board.clear();
    }
}

impl Default for TicTacToeGame {
    fn default() -> Self {
        TicTacToeGame {
            board: Default::default(),
            player1: Player {
                p_type: PlayerType::Human,
                name: "Player 1".to_string(),
                square_symbol: 'X',
            },
            player2: Player {
                p_type: PlayerType::Human,
                name: "Player 2".to_string(),
                square_symbol: 'O',
            },
            game_state: GameState::Begin(1),
            show_turn_info_mode: false,
        }
    }
}
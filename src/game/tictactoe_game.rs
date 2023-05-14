use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crate::game::ai::tictactoe_ai_player::Ai;
use crate::game::tictactoe_core::{SquareState, TicTacToe, TurnState};

pub struct TicTacToeGame {
    board: TicTacToe,
    player1: Player,
    player2: Player,
    game_state: GameState,
}

#[derive(Clone)]
enum PlayerType {
    Human,
    Computer(Ai)
}

#[derive(Clone)]
struct Player {
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

impl TicTacToeGame {
    pub fn load_default_1player_game(ai_difficulty: AiDifficulties) -> TicTacToeGame {
        let mut g: TicTacToeGame = Default::default();
        g.player2.p_type = PlayerType::Computer(TicTacToeGame::create_ai(ai_difficulty, g.player2.square_symbol.clone(), g.player1.square_symbol.clone()));
        return g
    }
    pub fn load_default_2player_game() -> TicTacToeGame {
        Default::default()
    }
    pub fn load_default_ai_game(ai1_difficulty: AiDifficulties, ai2_difficulty: AiDifficulties) -> TicTacToeGame {
        let mut g: TicTacToeGame = Default::default();
        g.player1.p_type = PlayerType::Computer(TicTacToeGame::create_ai(ai1_difficulty, g.player1.square_symbol.clone(), g.player2.square_symbol.clone()));
        g.player2.p_type = PlayerType::Computer(TicTacToeGame::create_ai(ai2_difficulty, g.player2.square_symbol.clone(), g.player1.square_symbol.clone()));
        return g
    }
    
    fn create_ai(difficulty: AiDifficulties, symbol: char, op_symbol: char) -> Ai {
        let max_childs: isize;
        let max_layers: isize;
        match difficulty {
            AiDifficulties::Easy => {
                max_childs = 2;
                max_layers = 4;
            }
            AiDifficulties::Medium => {
                max_childs = 5;
                max_layers = 10;
            }
            AiDifficulties::Hard => {
                max_childs = -1;
                max_layers = -1;
            }
        }
        Ai::create(max_childs, max_layers, symbol, op_symbol)
    }
    
    pub fn start_game(&mut self) {
        loop {
            match &self.game_state {
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
                    match p.p_type.clone() {
                        PlayerType::Human => {
                            println!("{}'s turn, type the column of your next move\ncolumn: ", p.name);
                            let mut ans: String = String::new();
                            stdout().flush().expect("");
                            stdin().read_line(&mut ans).unwrap();
                            ans.remove(ans.len()-1);
                            col = ans.parse().unwrap();
                            println!("\nline: ");
                            ans.clear();
                            stdout().flush().expect("");
                            stdin().read_line(&mut ans).unwrap();
                            ans.remove(ans.len()-1);
                            line = ans.parse().unwrap();
                        }
                        PlayerType::Computer(ref mut ai) => {
                            let ai_action = ai.act(self.board.clone());
                            col = ai_action.0 + 1;
                            line = ai_action.1 + 1;
                        }
                    }
                    match self.board.set_square(col - 1, line - 1, SquareState::Filled(p.square_symbol)) {
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
                            if n.eq(&1) {
                                self.set_current_player_to_2()
                            }else {
                                self.set_current_player_to_1()
                            }
                            println!("{}", self);
                        }
                    }
                    sleep(Duration::from_millis(500))
                }
                GameState::Finished => {
                    println!("\n{}", self.board);
                    println!("\nGame is finished! please re-load the game!");
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
        if let PlayerType::Computer(mut ai) = self.player1.p_type.clone() {
            ai.symbol = symbol
        }
    }
    pub fn set_player2_symbol(&mut self, symbol: char) {
        self.player2.square_symbol = symbol;
        if let PlayerType::Computer(mut ai) = self.player2.p_type.clone() {
            ai.symbol = symbol
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
            panic!("Invalid player number!")
        }
    }

    pub fn change_size(&mut self, size: usize, in_a_row_to_win: usize) {
        self.board.x_size = size;
        self.board.y_size = size;
        self.board.seq_to_win = in_a_row_to_win;
        self.board.squares = vec![SquareState::None; size.pow(2)]
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
        }
    }
}
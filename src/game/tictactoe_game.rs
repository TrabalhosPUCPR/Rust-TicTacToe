use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};
use crate::game::tictactoe_core::{SquareState, TicTacToe, TurnState};

pub struct TicTacToeGame {
    board: TicTacToe,
    player1: (PlayerType, String),
    player2: (PlayerType, String),
    game_state: GameState,
}

#[derive(Copy, Clone, PartialEq)]
enum PlayerType {
    Human,
    Computer
}

enum GameState {
    Player(PlayerType, String, SquareState),
    Finished,
    Begin
}

impl Display for TicTacToeGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let state = &self.game_state;
        return match state {
            GameState::Player(_, s, _) => {
                write!(f, "\nPlayer turn: {}\n\n", s).and(self.board.fmt(f))
            }
            GameState::Finished => {
                write!(f, "Game if finished!").and(self.board.fmt(f))
            }
            GameState::Begin => {
                write!(f, "Game have not started!").and(self.board.fmt(f))
            }
        }
    }
}

impl TicTacToeGame {
    pub fn load_default_1player_game() -> TicTacToeGame {
        let mut g: TicTacToeGame = Default::default();
        g.player2 = (PlayerType::Computer, String::from("Player 2"));
        return g
    }
    pub fn load_default_2player_game() -> TicTacToeGame {
        Default::default()
    }
    pub fn load_default_ai_game() -> TicTacToeGame {
        let mut g: TicTacToeGame = Default::default();
        g.player1 = (PlayerType::Computer, String::from("Player 1"));
        g.player2 = (PlayerType::Computer, String::from("Player 2"));
        return g
    }
    pub fn start_game(&mut self) {
        loop {
            match &self.game_state {
                GameState::Begin => {
                    self.set_current_player_to_1();
                    println!("{}", self)
                }
                GameState::Player(ptype, name, square) => {
                    let col: usize;
                    let line: usize;
                    match ptype {
                        PlayerType::Human => {
                            println!("{}'s turn, type the column of your next move\ncolumn: ", name);
                            let mut ans: String = String::new();
                            stdout().flush();
                            stdin().read_line(&mut ans).unwrap();
                            ans.remove(ans.len()-1);
                            col = ans.parse().unwrap();
                            println!("\nline: ");
                            ans.clear();
                            stdout().flush();
                            stdin().read_line(&mut ans).unwrap();
                            ans.remove(ans.len()-1);
                            line = ans.parse().unwrap();
                        }
                        PlayerType::Computer => {
                            line = 1;
                            col = 1;
                        }
                    }
                    match self.board.set_square(col - 1, line - 1, square.clone()) {
                        TurnState::Draw => {
                            println!("\nIt's a draw!");
                            self.game_state = GameState::Finished
                        }
                        TurnState::Error => {
                            println!("\nType a valid position!")
                        }
                        TurnState::Victory => {
                            println!("\nGame finished! {} wins!", name);
                            self.game_state = GameState::Finished
                        }
                        _ => {
                            match square {
                                SquareState::X => {
                                    self.set_current_player_to_2()
                                }
                                SquareState::O => {
                                    self.set_current_player_to_1()
                                }
                                _ => {}
                            }
                            println!("{}", self)
                        }
                    }
                }
                GameState::Finished => {
                    println!("{}", self.board);
                    println!("Game is finished! please re-load the game!");
                    return;
                }
            }
        }
    }
    fn set_current_player_to_1(&mut self) {
        self.game_state = GameState::Player(self.player1.0, self.player1.1.clone(), SquareState::X)
    }
    fn set_current_player_to_2(&mut self) {
        self.game_state = GameState::Player(self.player2.0, self.player2.1.clone(), SquareState::O)
    }
    pub fn set_empty_space_symbol(&mut self, symbol: char) {
        self.board.empty_space_symbol = symbol;
    }
    pub fn set_first_player(&mut self, player_n: usize){
        match player_n {
            1 => {
                self.set_current_player_to_1()
            }
            2 => {
                self.set_current_player_to_2()
            }
            n => {
                panic!("{} is not a valid player number", n)
            }
        }
    }

    pub fn change_size(&mut self, size: usize, in_a_row_to_win: usize) {
        self.board.x_size = size;
        self.board.y_size = size;
        self.board.seq_to_win = in_a_row_to_win;
    }
}

impl Default for TicTacToeGame {
    fn default() -> Self {
        TicTacToeGame {
            board: Default::default(),
            player1: (PlayerType::Human, String::from("Player 1")),
            player2: (PlayerType::Human, String::from("Player 2")),
            game_state: GameState::Begin,
        }
    }
}
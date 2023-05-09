use crate::game::tictactoe_game::TicTacToeGame;
use crate::game::ai::*;

mod game;

fn main() {
    let mut game: TicTacToeGame = TicTacToeGame::load_default_2player_game();

    game.start_game()
}





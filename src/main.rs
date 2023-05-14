use std::thread;
use std::time::Duration;
use crate::game::tictactoe_game::{AiDifficulties, TicTacToeGame};
mod game;

const SIZE: usize = 3;

fn main() {
    let mut game: TicTacToeGame;
    //game: TicTacToeGame = TicTacToeGame::load_default_2player_game();
    //game = TicTacToeGame::load_default_1player_game(AiDifficulties::Hard);
    game = TicTacToeGame::load_default_ai_game(AiDifficulties::Hard, AiDifficulties::Hard);
    game.change_size(SIZE, SIZE);

    loop {
        game.start_game();
        thread::sleep(Duration::from_millis(5000));
        println!("\n\nRestarting game\n\n");
        thread::sleep(Duration::from_millis(5000));
        game.reload_game();
    }
}





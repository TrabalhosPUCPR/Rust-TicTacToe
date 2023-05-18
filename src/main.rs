use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crate::game::tictactoe_game::{AiDifficulties, TicTacToeGame};
mod game;

fn main() {
    /*
    let mut game: TicTacToeGame;
    //game: TicTacToeGame = TicTacToeGame::load_default_2player_game();
    game = TicTacToeGame::load_default_1player_game(AiDifficulties::Hard);
    //game = TicTacToeGame::load_default_ai_game(AiDifficulties::Hard, AiDifficulties::Hard);
    game.change_size(10, 4);
    game.set_first_player(1);
    game.show_turn_info_mode = true;

    loop {
        game.start_game();
        sleep(Duration::from_secs(5));
        game.reload_game()
    }*/
    TicTacToeGame::start_new_game_prompts()
}




